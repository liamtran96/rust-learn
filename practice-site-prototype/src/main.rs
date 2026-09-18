//! PROTOTYPE: a dependency-free local server for the Rust Recall Lab UI.

use std::{
    collections::BTreeMap,
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    path::{Path, PathBuf},
    time::UNIX_EPOCH,
};

const ADDRESS: &str = "127.0.0.1:4173";

#[derive(Default)]
struct Mistake {
    id: String,
    chapter: String,
    date: String,
    title: String,
    wrote: String,
    why: String,
    rule: String,
    status: String,
    source: String,
}

#[derive(Default)]
struct JournalQuestion {
    id: String,
    chapter: String,
    topic: String,
    date: String,
    session: String,
    working_on: String,
    what_didnt: String,
    question: String,
    prompt_context: String,
    prompt_code: String,
    old_answer: String,
    answer: String,
    example: String,
    see_also: String,
    source: String,
}

#[derive(Clone, Copy, Default)]
enum Field {
    Wrote,
    Why,
    Rule,
    Status,
    #[default]
    None,
}

#[derive(Clone, Copy, Default)]
enum QuestionField {
    PromptContext,
    PromptCode,
    OldAnswer,
    Answer,
    Example,
    SeeAlso,
    #[default]
    None,
}

fn main() -> std::io::Result<()> {
    let site_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let repo_dir = site_dir
        .parent()
        .expect("prototype must remain directly inside the repository")
        .to_path_buf();
    let listener = TcpListener::bind(ADDRESS)?;
    println!("Rust practice prototype: http://{ADDRESS}");
    println!("Press Ctrl+C to stop.");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                if let Err(error) = handle_request(&mut stream, &site_dir, &repo_dir) {
                    eprintln!("Request failed: {error}");
                }
            }
            Err(error) => eprintln!("Connection failed: {error}"),
        }
    }
    Ok(())
}

fn handle_request(stream: &mut TcpStream, site_dir: &Path, repo_dir: &Path) -> std::io::Result<()> {
    let mut buffer = [0; 4096];
    let size = stream.read(&mut buffer)?;
    let request = String::from_utf8_lossy(&buffer[..size]);
    let target = request
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .unwrap_or("/")
        .split('?')
        .next()
        .unwrap_or("/");

    if target == "/api/practice" {
        let body = build_payload(repo_dir);
        return respond(
            stream,
            "200 OK",
            "application/json; charset=utf-8",
            body.as_bytes(),
        );
    }

    let (path, content_type) = match target {
        "/" | "/index.html" => (site_dir.join("index.html"), "text/html; charset=utf-8"),
        "/app.js" => (site_dir.join("app.js"), "text/javascript; charset=utf-8"),
        "/styles.css" => (site_dir.join("styles.css"), "text/css; charset=utf-8"),
        _ => return respond(stream, "404 Not Found", "text/plain", b"Not found"),
    };
    match fs::read(path) {
        Ok(body) => respond(stream, "200 OK", content_type, &body),
        Err(_) => respond(stream, "404 Not Found", "text/plain", b"Not found"),
    }
}

fn respond(
    stream: &mut TcpStream,
    status: &str,
    content_type: &str,
    body: &[u8],
) -> std::io::Result<()> {
    write!(
        stream,
        "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nCache-Control: no-store\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    )?;
    stream.write_all(body)
}

fn build_payload(repo_dir: &Path) -> String {
    let rust_dir = repo_dir.join("topics/rust");
    let progress_path = rust_dir.join("progress.md");
    let plan_path = rust_dir.join("study-plan.md");
    let mut mistake_paths = Vec::new();
    find_mistake_files(&rust_dir, &mut mistake_paths);
    mistake_paths.sort();
    let journal_dir = rust_dir.join("journal");
    let mut journal_paths = Vec::new();
    find_markdown_files(&journal_dir, &mut journal_paths);
    journal_paths.sort();

    let progress = fs::read_to_string(&progress_path).unwrap_or_default();
    let plan = fs::read_to_string(&plan_path).unwrap_or_default();
    let summary = parse_summary(&progress);
    let (completed, total, next_tasks) = parse_plan(&plan);
    let mut mistakes: Vec<Mistake> = mistake_paths
        .iter()
        .flat_map(|path| parse_mistakes(path, repo_dir))
        .collect();
    mistakes.sort_by(|a, b| b.date.cmp(&a.date).then(a.chapter.cmp(&b.chapter)));
    let mut questions: Vec<JournalQuestion> = journal_paths
        .iter()
        .flat_map(|path| parse_journal_questions(path, repo_dir))
        .collect();
    let mut topic_start_dates = BTreeMap::new();
    for item in &questions {
        topic_start_dates
            .entry(item.source.clone())
            .and_modify(|date: &mut String| {
                if item.date < *date {
                    date.clone_from(&item.date);
                }
            })
            .or_insert_with(|| item.date.clone());
    }
    questions.sort_by(|a, b| {
        chapter_rank(&a.source)
            .cmp(&chapter_rank(&b.source))
            .then(topic_start_dates[&a.source].cmp(&topic_start_dates[&b.source]))
            .then(a.source.cmp(&b.source))
            .then(a.date.cmp(&b.date))
    });

    let version = [
        vec![progress_path, plan_path],
        mistake_paths.clone(),
        journal_paths.clone(),
    ]
    .concat()
    .iter()
    .filter_map(|path| fs::metadata(path).ok()?.modified().ok())
    .filter_map(|time| time.duration_since(UNIX_EPOCH).ok())
    .map(|duration| duration.as_nanos())
    .max()
    .unwrap_or_default();
    let sources = [
        vec![rust_dir.join("progress.md"), rust_dir.join("study-plan.md")],
        mistake_paths,
        journal_paths,
    ]
    .concat();

    format!(
        "{{\"version\":{},\"generatedFrom\":[{}],\"summary\":{{{}}},\"plan\":{{\"completed\":{},\"total\":{},\"nextTasks\":[{}]}},\"questions\":[{}],\"mistakes\":[{}]}}",
        json_string(&version.to_string()),
        sources
            .iter()
            .map(|path| json_string(&relative_source(path, repo_dir)))
            .collect::<Vec<_>>()
            .join(","),
        summary
            .iter()
            .map(|(key, value)| format!("{}:{}", json_string(key), json_string(value)))
            .collect::<Vec<_>>()
            .join(","),
        completed,
        total,
        next_tasks
            .iter()
            .map(|task| json_string(task))
            .collect::<Vec<_>>()
            .join(","),
        questions
            .iter()
            .map(question_json)
            .collect::<Vec<_>>()
            .join(","),
        mistakes
            .iter()
            .map(mistake_json)
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn find_markdown_files(directory: &Path, result: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(directory) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            find_markdown_files(&path, result);
        } else if path.extension().is_some_and(|extension| extension == "md") {
            result.push(path);
        }
    }
}

fn find_mistake_files(directory: &Path, result: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(directory) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            find_mistake_files(&path, result);
        } else if path.file_name().is_some_and(|name| name == "mistakes.md") {
            result.push(path);
        }
    }
}

fn parse_summary(text: &str) -> BTreeMap<String, String> {
    let mut summary = BTreeMap::new();
    let mut in_summary = false;
    for line in text.lines() {
        if line.trim() == "## Summary" {
            in_summary = true;
            continue;
        }
        if in_summary && line.starts_with("## ") {
            break;
        }
        if !in_summary || !line.starts_with('|') {
            continue;
        }
        let cells: Vec<_> = line.trim_matches('|').split('|').map(str::trim).collect();
        if cells.len() >= 2 && cells[0] != "Metric" && !cells[0].starts_with("---") {
            summary.insert(clean_markdown(cells[0]), clean_markdown(cells[1]));
        }
    }
    summary
}

fn parse_plan(text: &str) -> (usize, usize, Vec<String>) {
    let mut completed = 0;
    let mut total = 0;
    let mut next_tasks = Vec::new();
    for line in text.lines() {
        let trimmed = line.trim_start();
        let mark = if trimmed.starts_with("- [x]") || trimmed.starts_with("- [X]") {
            Some(true)
        } else if trimmed.starts_with("- [ ]") {
            Some(false)
        } else {
            None
        };
        if let Some(done) = mark {
            total += 1;
            completed += usize::from(done);
            if !done && next_tasks.len() < 4 {
                next_tasks.push(clean_markdown(trimmed.get(5..).unwrap_or_default()));
            }
        }
    }
    (completed, total, next_tasks)
}

fn parse_mistakes(path: &Path, repo_dir: &Path) -> Vec<Mistake> {
    let Ok(text) = fs::read_to_string(path) else {
        return Vec::new();
    };
    let chapter = chapter_display(
        path.parent()
            .and_then(Path::file_name)
            .and_then(|name| name.to_str())
            .unwrap_or("Rust"),
    );
    let source = relative_source(path, repo_dir);
    let mut result = Vec::new();
    let mut current: Option<Mistake> = None;
    let mut field = Field::None;
    let mut in_open = false;
    let mut in_code = false;

    for line in text.lines() {
        if line.starts_with("## Open mistakes") {
            in_open = true;
            continue;
        }
        if in_open && line.starts_with("## Resolved") {
            finish_mistake(&mut current, &mut result);
            break;
        }
        if !in_open {
            continue;
        }
        if let Some(heading) = line.strip_prefix("### ") {
            finish_mistake(&mut current, &mut result);
            let cleaned = clean_markdown(heading);
            let (date, title) = if cleaned.len() >= 10 && cleaned.as_bytes().get(4) == Some(&b'-') {
                (
                    cleaned[..10].to_string(),
                    cleaned[10..]
                        .trim_start_matches([' ', '-', '–', '—'])
                        .trim()
                        .to_string(),
                )
            } else {
                (String::new(), cleaned)
            };
            current = Some(Mistake {
                chapter: chapter.clone(),
                date,
                title,
                source: source.clone(),
                ..Mistake::default()
            });
            field = Field::None;
            continue;
        }
        let Some(item) = current.as_mut() else {
            continue;
        };
        if line.trim_start().starts_with("```") {
            in_code = !in_code;
            continue;
        }
        if in_code {
            continue;
        }
        if let Some(value) = labelled_value(line, "What I wrote") {
            field = Field::Wrote;
            append_field(item, field, value);
        } else if let Some(value) = labelled_value(line, "Why it's wrong") {
            field = Field::Why;
            append_field(item, field, value);
        } else if let Some(value) = labelled_value(line, "The rule") {
            field = Field::Rule;
            append_field(item, field, value);
        } else if let Some(value) = labelled_value(line, "Status") {
            append_field(item, Field::Status, value);
            field = Field::None;
        } else if line.starts_with("- **") || line.starts_with("**") || line.starts_with('#') {
            field = Field::None;
        } else if !line.trim().is_empty()
            && !line.contains("flowchart")
            && !line.contains("sequenceDiagram")
        {
            append_field(item, field, line.trim_start_matches([' ', '-', '*']).trim());
        }
    }
    finish_mistake(&mut current, &mut result);
    for (index, item) in result.iter_mut().enumerate() {
        item.id = format!(
            "{}-{}",
            path.parent()
                .and_then(Path::file_name)
                .and_then(|name| name.to_str())
                .unwrap_or("rust"),
            index + 1
        );
    }
    result
}

fn labelled_value<'a>(line: &'a str, label: &str) -> Option<&'a str> {
    let prefix = format!("- **{label}:**");
    line.trim_start().strip_prefix(&prefix).map(str::trim)
}

fn append_field(item: &mut Mistake, field: Field, value: &str) {
    let target = match field {
        Field::Wrote => &mut item.wrote,
        Field::Why => &mut item.why,
        Field::Rule => &mut item.rule,
        Field::Status => &mut item.status,
        Field::None => return,
    };
    if !value.is_empty() {
        if !target.is_empty() {
            target.push(' ')
        }
        target.push_str(&clean_markdown(value));
    }
}

fn finish_mistake(current: &mut Option<Mistake>, result: &mut Vec<Mistake>) {
    if let Some(item) = current.take()
        && (!item.rule.is_empty() || !item.why.is_empty())
    {
        result.push(item);
    }
}

fn parse_journal_questions(path: &Path, repo_dir: &Path) -> Vec<JournalQuestion> {
    let Ok(text) = fs::read_to_string(path) else {
        return Vec::new();
    };
    let chapter = chapter_display(
        path.parent()
            .and_then(Path::file_name)
            .and_then(|name| name.to_str())
            .unwrap_or("Rust"),
    );
    let source = relative_source(path, repo_dir);
    let topic = text
        .lines()
        .find_map(|line| line.strip_prefix("# "))
        .map(clean_markdown)
        .unwrap_or_else(|| {
            path.file_stem()
                .and_then(|name| name.to_str())
                .unwrap_or("Rust")
                .replace('-', " ")
        })
        .trim_end_matches(" Journal")
        .to_string();
    let mut result = Vec::new();
    let mut current: Option<JournalQuestion> = None;
    let mut date = String::new();
    let mut session = String::new();
    let mut working_on = String::new();
    let mut what_didnt = String::new();
    let mut in_entries = false;
    let mut in_questions = false;
    let mut field = QuestionField::None;
    let mut in_code = false;

    for line in text.lines() {
        if line.trim() == "## Entries" {
            in_entries = true;
            continue;
        }
        if !in_entries {
            continue;
        }
        if let Some(heading) = line.strip_prefix("### ") {
            finish_question(&mut current, &mut result);
            let cleaned = clean_markdown(heading);
            if cleaned.len() >= 10 && cleaned.as_bytes().get(4) == Some(&b'-') {
                date = cleaned[..10].to_string();
                session = cleaned[10..]
                    .trim_start_matches([' ', '-', '–', '—'])
                    .trim()
                    .to_string();
            } else {
                date.clear();
                session = cleaned;
            }
            working_on.clear();
            what_didnt.clear();
            in_questions = false;
            field = QuestionField::None;
            continue;
        }
        if let Some(value) = bold_value(line, "Working on") {
            working_on = clean_markdown(value);
            continue;
        }
        if let Some(value) = bold_value(line, "What didn't") {
            what_didnt = clean_markdown(value);
            continue;
        }
        if line.starts_with("**Questions asked this session:**") {
            in_questions = true;
            continue;
        }
        if in_questions
            && (line.starts_with("**Question to answer later:")
                || line.starts_with("**Verification:")
                || line.starts_with("**Next:"))
        {
            finish_question(&mut current, &mut result);
            in_questions = false;
            field = QuestionField::None;
            continue;
        }
        if !in_questions {
            continue;
        }
        if let Some(raw_question) = line.trim_start().strip_prefix("- **Q:**") {
            finish_question(&mut current, &mut result);
            let (question, old_answer) = split_initial_answer(&clean_markdown(raw_question));
            current = Some(JournalQuestion {
                chapter: chapter.clone(),
                topic: topic.clone(),
                date: date.clone(),
                session: session.clone(),
                working_on: working_on.clone(),
                what_didnt: what_didnt.clone(),
                question,
                old_answer,
                source: source.clone(),
                ..JournalQuestion::default()
            });
            field = QuestionField::None;
            continue;
        }
        if let Some((question, answer)) = parse_legacy_question(line)
            && is_reviewable_question(&question)
        {
            finish_question(&mut current, &mut result);
            result.push(JournalQuestion {
                chapter: chapter.clone(),
                topic: topic.clone(),
                date: date.clone(),
                session: session.clone(),
                working_on: working_on.clone(),
                what_didnt: what_didnt.clone(),
                question,
                answer,
                source: source.clone(),
                ..JournalQuestion::default()
            });
            continue;
        }
        let Some(item) = current.as_mut() else {
            continue;
        };
        if line.trim_start().starts_with("```") {
            in_code = !in_code;
            continue;
        }
        if let Some(value) = nested_value(line, "Prompt context") {
            field = QuestionField::PromptContext;
            append_question_field(item, field, value);
        } else if let Some(value) = nested_value(line, "Prompt code") {
            field = QuestionField::PromptCode;
            append_question_field(item, field, value);
        } else if let Some(value) = nested_value(line, "Liam's answer") {
            field = QuestionField::OldAnswer;
            item.old_answer.clear();
            append_question_field(item, field, value);
        } else if let Some(value) = nested_value(line, "Technical answer") {
            field = QuestionField::Answer;
            append_question_field(item, field, value);
        } else if let Some(value) = nested_value(line, "Plain-English analogy / example") {
            field = QuestionField::Example;
            append_question_field(item, field, value);
        } else if let Some(value) = nested_value(line, "See also") {
            field = QuestionField::SeeAlso;
            append_question_field(item, field, value);
        } else if in_code {
            append_question_field(item, field, line.trim());
        } else if !line.trim().is_empty() && line.starts_with("    ") {
            append_question_field(item, field, line.trim_start_matches([' ', '-']).trim());
        }
    }
    finish_question(&mut current, &mut result);
    for (index, item) in result.iter_mut().enumerate() {
        let stem = path
            .file_stem()
            .and_then(|name| name.to_str())
            .unwrap_or("journal");
        item.id = format!("journal-{stem}-{}", index + 1);
    }
    result
}

fn bold_value<'a>(line: &'a str, label: &str) -> Option<&'a str> {
    line.strip_prefix(&format!("**{label}:**")).map(str::trim)
}

fn nested_value<'a>(line: &'a str, label: &str) -> Option<&'a str> {
    line.trim_start()
        .strip_prefix(&format!("- **{label}:**"))
        .map(str::trim)
}

fn split_initial_answer(value: &str) -> (String, String) {
    for marker in [" Initial answer:", " initial answer:"] {
        if let Some((question, answer)) = value.split_once(marker) {
            return (
                question.trim().to_string(),
                answer.trim().trim_matches(['"', ' ', '.']).to_string(),
            );
        }
    }
    (value.to_string(), String::new())
}

fn parse_legacy_question(line: &str) -> Option<(String, String)> {
    let trimmed = line.trim_start();
    let content = trimmed.strip_prefix("- *")?;
    let question_end = content.find('*')?;
    let question = &content[..question_end];
    let remainder = content[question_end + 1..].trim_start();
    let remainder = remainder.trim_start_matches(['-', '–', '—']).trim_start();
    let answer = remainder.strip_prefix("answered:")?;
    Some((clean_markdown(question), clean_markdown(answer)))
}

fn append_question_field(item: &mut JournalQuestion, field: QuestionField, value: &str) {
    let target = match field {
        QuestionField::PromptContext => &mut item.prompt_context,
        QuestionField::PromptCode => &mut item.prompt_code,
        QuestionField::OldAnswer => &mut item.old_answer,
        QuestionField::Answer => &mut item.answer,
        QuestionField::Example => &mut item.example,
        QuestionField::SeeAlso => &mut item.see_also,
        QuestionField::None => return,
    };
    let value = value.trim();
    if !value.is_empty() && value != "-" && value != "->" {
        if !target.is_empty() {
            target.push('\n');
        }
        if matches!(field, QuestionField::PromptCode) {
            target.push_str(value.trim_matches('`'));
        } else {
            target.push_str(&clean_markdown(value));
        }
    }
}

fn finish_question(current: &mut Option<JournalQuestion>, result: &mut Vec<JournalQuestion>) {
    if let Some(item) = current.take()
        && !item.question.is_empty()
        && !item.answer.is_empty()
        && is_reviewable_question(&item.question)
    {
        result.push(item);
    }
}

fn is_reviewable_question(question: &str) -> bool {
    let normalized = question.to_ascii_lowercase().replace('’', "'");
    ![
        "i dont understand your question",
        "i don't understand your question",
        "still dont understand your question",
        "still don't understand your question",
        "what should i do next",
        "the hell what u mean",
        "check it",
        "still error",
        "what do you mean by number",
        "what do u mean by number",
        "you mean i should write a test",
        "give me the hint",
    ]
    .iter()
    .any(|fragment| normalized.contains(fragment))
}

fn chapter_display(folder: &str) -> String {
    let words = folder
        .trim_start_matches(|character: char| character.is_ascii_digit() || character == '-')
        .replace('-', " ");
    let mut characters = words.chars();
    match characters.next() {
        Some(first) => format!("{}{}", first.to_uppercase(), characters.as_str()),
        None => "Rust".to_string(),
    }
}

fn chapter_rank(source: &str) -> u32 {
    source
        .split('/')
        .find(|part| {
            part.chars()
                .next()
                .is_some_and(|character| character.is_ascii_digit())
        })
        .and_then(|part| part.split('-').next())
        .and_then(|prefix| prefix.parse().ok())
        .unwrap_or(u32::MAX)
}

fn clean_markdown(value: &str) -> String {
    let mut result = value.replace(['`', '*'], "");
    while let Some(start) = result.find("[[") {
        let Some(relative_end) = result[start + 2..].find("]]") else {
            break;
        };
        let end = start + 2 + relative_end;
        let inner = &result[start + 2..end];
        let label = inner
            .split('|')
            .next_back()
            .unwrap_or(inner)
            .split('#')
            .next()
            .unwrap_or(inner)
            .to_string();
        result.replace_range(start..end + 2, &label);
    }
    result
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .trim_matches([' ', '-'])
        .to_string()
}

fn relative_source(path: &Path, repo_dir: &Path) -> String {
    path.strip_prefix(repo_dir)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn mistake_json(item: &Mistake) -> String {
    format!(
        "{{\"id\":{},\"chapter\":{},\"date\":{},\"title\":{},\"prompt\":{},\"whatIThought\":{},\"explanation\":{},\"rule\":{},\"status\":{},\"source\":{}}}",
        json_string(&item.id),
        json_string(&item.chapter),
        json_string(&item.date),
        json_string(&item.title),
        json_string(
            "What is incorrect or incomplete in the earlier attempt? Explain what Rust does, why, and how to correct it.",
        ),
        json_string(&item.wrote),
        json_string(&item.why),
        json_string(&item.rule),
        json_string(if item.status.is_empty() {
            "fresh"
        } else {
            &item.status
        }),
        json_string(&item.source)
    )
}

fn question_json(item: &JournalQuestion) -> String {
    format!(
        "{{\"id\":{},\"chapter\":{},\"topic\":{},\"date\":{},\"session\":{},\"workingOn\":{},\"whatDidnt\":{},\"question\":{},\"promptContext\":{},\"promptCode\":{},\"contextReady\":{},\"oldAnswer\":{},\"answer\":{},\"example\":{},\"seeAlso\":{},\"source\":{}}}",
        json_string(&item.id),
        json_string(&item.chapter),
        json_string(&item.topic),
        json_string(&item.date),
        json_string(&item.session),
        json_string(&item.working_on),
        json_string(&item.what_didnt),
        json_string(&item.question),
        json_string(&item.prompt_context),
        json_string(&item.prompt_code),
        !item.prompt_context.is_empty(),
        json_string(&item.old_answer),
        json_string(&item.answer),
        json_string(&item.example),
        json_string(&item.see_also),
        json_string(&item.source)
    )
}

fn json_string(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len() + 2);
    escaped.push('"');
    for character in value.chars() {
        match character {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            character if character.is_control() => escaped.push(' '),
            character => escaped.push(character),
        }
    }
    escaped.push('"');
    escaped
}
