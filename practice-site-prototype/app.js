// PROTOTYPE: Three Rust practice layouts, switchable with ?variant=A|B|C.
const variants = { A: "Focus session", B: "Review dashboard", C: "Learning trail" };
const state = {
  data: null,
  version: null,
  activeIndex: 0,
  revealed: false,
  ratings: new Map(),
  answers: new Map(),
  filter: "All",
};
const app = document.querySelector("#app");

function escapeHtml(value = "") {
  return String(value).replace(/[&<>'"]/g, (character) => ({
    "&": "&amp;", "<": "&lt;", ">": "&gt;", "'": "&#39;", '"': "&quot;",
  })[character]);
}

function currentVariant() {
  const candidate = new URLSearchParams(window.location.search).get("variant")?.toUpperCase();
  return variants[candidate] ? candidate : "A";
}

function setVariant(key) {
  const url = new URL(window.location.href);
  url.searchParams.set("variant", key);
  history.replaceState({}, "", url);
  render();
}

function cycleVariant(direction) {
  const keys = Object.keys(variants);
  const current = keys.indexOf(currentVariant());
  setVariant(keys[(current + direction + keys.length) % keys.length]);
}

function summaryValue(name, fallback = "-") {
  return state.data.summary[name] || fallback;
}

function topicOptions() {
  return ["All", ...new Set(state.data.questions.map((item) => item.topic))];
}

function filteredQuestions() {
  if (state.filter === "All") return state.data.questions;
  return state.data.questions.filter((item) => item.topic === state.filter || item.chapter === state.filter);
}

function activeQuestion() {
  const items = filteredQuestions();
  if (!items.length) return null;
  state.activeIndex %= items.length;
  return items[state.activeIndex];
}

function statusChip(item) {
  const rating = state.ratings.get(item.id);
  const status = item.contextReady ? "context ready" : "historical";
  return `<span class="chip ${rating ? `rated-${rating}` : item.contextReady ? "context-ready" : "context-missing"}">${escapeHtml(rating || status)}</span>`;
}

function sourceLink(item) {
  return `<span class="source">Source: ${escapeHtml(item.source)}</span>`;
}

function reviewExercise(item) {
  return `
    <section class="review-context">
      <div><span class="eyebrow">Context from your journal</span><span class="context-date">${escapeHtml(item.date)}</span></div>
      <p><strong>${escapeHtml(item.session)}</strong></p>
      ${item.workingOn ? `<p>Working on: ${escapeHtml(item.workingOn)}</p>` : ""}
    </section>
    ${item.contextReady ? `<section class="prompt-packet">
      <span class="eyebrow">Original prompt</span>
      <p>${escapeHtml(item.promptContext)}</p>
      ${item.promptCode ? `<pre><code>${escapeHtml(item.promptCode)}</code></pre>` : `<div class="no-code">Conceptual question - no code was involved.</div>`}
    </section>` : `<section class="context-missing-note"><span class="eyebrow">Historical context incomplete</span><p>This older journal entry did not preserve the original prompt or pre-answer code. The session, question, journal answer, and related notes are still available.</p></section>`}
    <section class="review-task">
      <span class="eyebrow">Question from that session</span>
      <h2>${escapeHtml(item.question)}</h2>
      <p>Answer it again from memory before revealing the explanation recorded in your journal.</p>
    </section>
    ${item.whatDidnt ? `<details class="review-hint"><summary>Need a context hint?</summary><p>From "What didn't" in that session: ${escapeHtml(item.whatDidnt)}</p></details>` : ""}`;
}

function answerComposer(item) {
  const answer = state.answers.get(item.id) || "";
  return `
    <section class="answer-composer">
      <label for="answer-${escapeHtml(item.id)}">Your answer now</label>
      <textarea id="answer-${escapeHtml(item.id)}" data-answer-input="${escapeHtml(item.id)}" rows="5" placeholder="Answer the session question in your own words...">${escapeHtml(answer)}</textarea>
      <span>Stored only for this browser session.</span>
    </section>`;
}

function answerPanel(item) {
  if (!state.revealed) return "";
  const answer = state.answers.get(item.id)?.trim();
  return `
    <section class="answer-panel">
      <div class="answer-comparison">
        <div class="my-answer"><span class="eyebrow">Your answer now</span><p>${answer ? escapeHtml(answer) : "No answer entered."}</p></div>
        <div class="rule"><span class="eyebrow">Journal answer</span><p>${escapeHtml(item.answer)}</p></div>
      </div>
      ${item.oldAnswer ? `<div class="why-panel old-answer-review"><span class="eyebrow">Your answer in that session</span><p>${escapeHtml(item.oldAnswer)}</p></div>` : ""}
      ${item.example ? `<div class="why-panel"><span class="eyebrow">Example or analogy from the session</span><p>${escapeHtml(item.example)}</p></div>` : ""}
      ${item.seeAlso ? `<div class="see-also"><span class="eyebrow">Related notes</span><p>${escapeHtml(item.seeAlso)}</p></div>` : ""}
      <div class="rating-row" aria-label="Rate this recall">
        <span>How did that feel?</span>
        <button data-rate="again">Again</button>
        <button data-rate="shaky">Shaky</button>
        <button data-rate="got-it">Got it</button>
      </div>
    </section>`;
}

function renderHeader() {
  return `
    <header class="site-header">
      <a class="brand" href="?variant=${currentVariant()}"><span class="brand-mark">R</span><span>Recall Lab <small>prototype</small></span></a>
      <div class="live-status"><span class="live-dot"></span>Watching journals</div>
    </header>`;
}

function renderA() {
  const item = activeQuestion();
  const total = filteredQuestions().length;
  return `
    <div class="shell focus-shell">
      ${renderHeader()}
      <section class="focus-intro">
        <div><p class="overline">TODAY'S RETRIEVAL</p><h1>Revisit your real<br /><em>session questions.</em></h1></div>
        <div class="progress-copy"><strong>${state.activeIndex + 1}/${total}</strong><span>journal questions</span></div>
      </section>
      <div class="chapter-tabs">${topicOptions().map((topic) => `<button data-filter="${escapeHtml(topic)}" class="${topic === state.filter ? "active" : ""}">${escapeHtml(topic)}</button>`).join("")}</div>
      ${item ? `<article class="practice-card">
        <div class="card-meta"><span>${escapeHtml(item.chapter)}</span>${statusChip(item)}</div>
        ${reviewExercise(item)}
        ${answerComposer(item)}
        <div class="practice-actions">
          <button class="primary" data-action="reveal">${state.revealed ? "Hide answer" : "Reveal answer"}</button>
          <button class="quiet" data-action="skip">Next question &rarr;</button>
        </div>
        ${answerPanel(item)}
        <footer>${sourceLink(item)}<span>${escapeHtml(item.date)}</span></footer>
      </article>` : `<p>No journal questions in this filter yet.</p>`}
      <section class="micro-stats">
        <div><strong>${escapeHtml(summaryValue("Exercises completed"))}</strong><span>Exercises</span></div>
        <div><strong>${escapeHtml(summaryValue("Current phase"))}</strong><span>Current phase</span></div>
        <div><strong>${state.ratings.size}</strong><span>Reviewed now</span></div>
      </section>
    </div>`;
}

function queueRows(items) {
  return items.map((item, index) => `
    <button class="queue-row ${activeQuestion()?.id === item.id ? "active" : ""}" data-index="${index}">
      <span class="queue-number">${String(index + 1).padStart(2, "0")}</span>
      <span><strong>${escapeHtml(item.question)}</strong><small>${escapeHtml(item.topic)} · ${escapeHtml(item.session)}</small></span>
      ${statusChip(item)}
    </button>`).join("");
}

function renderB() {
  const item = activeQuestion();
  const items = filteredQuestions();
  const plan = state.data.plan;
  return `
    <div class="dashboard-shell">
      <aside class="dashboard-sidebar">
        <div class="brand"><span class="brand-mark">R</span><span>Recall Lab</span></div>
        <nav>${topicOptions().map((topic) => `<button data-filter="${escapeHtml(topic)}" class="${topic === state.filter ? "active" : ""}">${escapeHtml(topic)} <span>${topic === "All" ? state.data.questions.length : state.data.questions.filter((item) => item.topic === topic).length}</span></button>`).join("")}</nav>
        <div class="sidebar-foot"><span class="live-dot"></span>Auto-sync on</div>
      </aside>
      <section class="dashboard-main">
        <header class="dashboard-title"><div><p class="overline">REVIEW QUEUE</p><h1>Good evening, Liam.</h1></div><div class="date-block"><strong>${summaryValue("Current week")}</strong><span>${summaryValue("Last session")}</span></div></header>
        <div class="stat-strip">
          <div><span>Journal questions</span><strong>${state.data.questions.length}</strong></div>
          <div><span>Plan completion</span><strong>${plan.completed}/${plan.total}</strong></div>
          <div><span>Current streak</span><strong>${escapeHtml(summaryValue("Streak (current)"))}</strong></div>
        </div>
        <div class="dashboard-grid">
          <section class="queue-panel"><div class="panel-heading"><h2>Your sessions</h2><span>Beginning to current</span></div>${queueRows(items)}</section>
          <section class="review-panel">${item ? `<div class="card-meta"><span>${escapeHtml(item.chapter)}</span>${statusChip(item)}</div>${reviewExercise(item)}${answerComposer(item)}<button class="primary wide" data-action="reveal">${state.revealed ? "Hide journal answer" : "Compare with journal"}</button>${answerPanel(item)}${sourceLink(item)}` : "No review items."}</section>
        </div>
      </section>
    </div>`;
}

function renderC() {
  const chapters = [...new Set([...state.data.questions]
    .sort((left, right) => left.source.localeCompare(right.source))
    .map((item) => item.chapter))];
  const item = activeQuestion();
  return `
    <div class="shell trail-shell">
      ${renderHeader()}
      <section class="trail-hero"><p class="overline">PROGRESS IS A PATH, NOT A SCORE</p><h1>Revisit the questions<br />from your Rust journey.</h1><p>Every marker comes from a real question recorded during a learning session.</p></section>
      <section class="trail-layout">
        <div class="trail-line">
          ${chapters.map((chapter, chapterIndex) => {
            const questions = state.data.questions.filter((entry) => entry.chapter === chapter);
            return `<article class="trail-stop ${chapterIndex % 2 ? "right" : "left"}">
              <button class="trail-marker" data-filter="${escapeHtml(chapter)}" aria-label="Practice ${escapeHtml(chapter)}">${questions.length}</button>
              <div><span class="eyebrow">Chapter ${chapterIndex + 1}</span><h2>${escapeHtml(chapter)}</h2><p>${questions.length} session questions</p><button class="text-button" data-filter="${escapeHtml(chapter)}">Practice this chapter &rarr;</button></div>
            </article>`;
          }).join("")}
        </div>
        <aside class="trail-practice">
          <p class="overline">CURRENT SESSION QUESTION</p>
          ${item ? `${reviewExercise(item)}${answerComposer(item)}<button class="primary" data-action="reveal">${state.revealed ? "Hide journal answer" : "Compare with journal"}</button>${answerPanel(item)}<button class="quiet" data-action="skip">Walk to next question &rarr;</button>` : "Pick a chapter marker."}
        </aside>
      </section>
    </div>`;
}

function bindInteractions() {
  document.querySelectorAll("[data-answer-input]").forEach((input) => input.addEventListener("input", () => {
    state.answers.set(input.dataset.answerInput, input.value);
  }));
  document.querySelectorAll("[data-filter]").forEach((button) => button.addEventListener("click", () => {
    state.filter = button.dataset.filter;
    state.activeIndex = 0;
    state.revealed = false;
    render();
  }));
  document.querySelectorAll("[data-index]").forEach((button) => button.addEventListener("click", () => {
    state.activeIndex = Number(button.dataset.index);
    state.revealed = false;
    render();
  }));
  document.querySelectorAll("[data-action='reveal']").forEach((button) => button.addEventListener("click", () => {
    state.revealed = !state.revealed;
    render();
  }));
  document.querySelectorAll("[data-action='skip']").forEach((button) => button.addEventListener("click", nextQuestion));
  document.querySelectorAll("[data-rate]").forEach((button) => button.addEventListener("click", () => {
    const item = activeQuestion();
    if (item) state.ratings.set(item.id, button.dataset.rate);
    nextQuestion();
  }));
}

function nextQuestion() {
  state.activeIndex = (state.activeIndex + 1) % Math.max(filteredQuestions().length, 1);
  state.revealed = false;
  render();
}

function render() {
  if (!state.data) return;
  const variant = currentVariant();
  document.body.dataset.variant = variant;
  document.querySelector("#variant-label").textContent = `${variant} - ${variants[variant]}`;
  app.innerHTML = variant === "A" ? renderA() : variant === "B" ? renderB() : renderC();
  bindInteractions();
}

async function syncData() {
  try {
    const response = await fetch("/api/practice", { cache: "no-store" });
    if (!response.ok) throw new Error(`Server returned ${response.status}`);
    const data = await response.json();
    if (data.version !== state.version) {
      state.data = data;
      state.version = data.version;
      render();
    }
  } catch (error) {
    if (!state.data) app.innerHTML = `<div class="loading error">Could not load learning records.<br><small>${escapeHtml(error.message)}</small></div>`;
  }
}

document.querySelector("#previous-variant").addEventListener("click", () => cycleVariant(-1));
document.querySelector("#next-variant").addEventListener("click", () => cycleVariant(1));
window.addEventListener("keydown", (event) => {
  if (["INPUT", "TEXTAREA"].includes(document.activeElement.tagName) || document.activeElement.isContentEditable) return;
  if (event.key === "ArrowLeft") cycleVariant(-1);
  if (event.key === "ArrowRight") cycleVariant(1);
});
window.addEventListener("popstate", render);

syncData();
setInterval(syncData, 2500);
