const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

const state = {
  files: [],
  dest: null,
  configured: false,
  cpuCount: 4,
};

const el = (id) => document.getElementById(id);

function updateThreadBlurb() {
  const value = parseInt(el("thread-count").value, 10) || 1;
  const cpuCount = state.cpuCount;
  const hint = el("thread-hint");

  if (value <= 1) {
    hint.textContent = "1 = default — matches the underlying tool's own default, and is the safest choice.";
    hint.classList.remove("warn");
  } else if (value >= cpuCount) {
    hint.textContent = `⚠ At or above your CPU's core count (${cpuCount}) — might slow your system down.`;
    hint.classList.add("warn");
  } else if (value >= cpuCount - 1) {
    hint.textContent = `⚠ Nearing your CPU's core count (${cpuCount}) — might slow your system down.`;
    hint.classList.add("warn");
  } else {
    hint.textContent = "More threads usually won't speed up conversion — it's disk-bound, not CPU-bound.";
    hint.classList.remove("warn");
  }
}

function showView(id) {
  document.querySelectorAll(".view").forEach((v) => v.classList.remove("active"));
  el(id).classList.add("active");
}

function renderFilesSummary() {
  el("files-summary").textContent =
    state.files.length === 0
      ? "No files selected"
      : `${state.files.length} file${state.files.length > 1 ? "s" : ""} selected`;
  updateConvertEnabled();
}

function renderDestSummary() {
  el("dest-summary").textContent = state.dest || "No destination selected";
  updateConvertEnabled();
}

function updateConvertEnabled() {
  el("convert-btn").disabled = state.files.length === 0 || !state.dest;
}

async function openSetup(isFirstRun) {
  const s = await invoke("get_settings");
  el("setup-title").textContent = isFirstRun ? "Welcome to ISO to GOD Converter" : "Conversion Settings";
  el("setup-intro").textContent = isFirstRun
    ? "Let's set your default conversion options. You can change these again later from the settings button."
    : "Update your default conversion options below.";
  el("setup-cancel").hidden = isFirstRun;

  state.cpuCount = s.cpu_count;
  el("thread-count").value = s.thread_count;
  el("thread-count").min = 1;
  el("thread-count").removeAttribute("max");
  updateThreadBlurb();

  el("trim-from-end").checked = !s.trim_none;
  el("trim-none").checked = s.trim_none;

  el("freq-every-time").checked = s.ask_every_time;
  el("freq-once").checked = !s.ask_every_time;

  showView("setup-view");
}

async function saveSetup() {
  const threadCount = Math.max(1, parseInt(el("thread-count").value, 10) || 1);
  const trimNone = el("trim-none").checked;
  const askEveryTime = el("freq-every-time").checked;

  await invoke("save_settings", {
    askEveryTime,
    threadCount,
    trimNone,
  });

  showView("main-view");
}

async function init() {
  const s = await invoke("get_settings");
  state.configured = s.configured;

  if (!s.configured || s.ask_every_time) {
    await openSetup(!s.configured);
  } else {
    showView("main-view");
  }

  el("setup-save").addEventListener("click", saveSetup);
  el("setup-cancel").addEventListener("click", () => showView("main-view"));
  el("open-settings").addEventListener("click", () => openSetup(false));
  el("thread-count").addEventListener("input", updateThreadBlurb);

  el("pick-files").addEventListener("click", async () => {
    const picked = await invoke("pick_iso_files");
    if (picked) {
      state.files = picked;
      renderFilesSummary();
    }
  });

  el("pick-dest").addEventListener("click", async () => {
    const picked = await invoke("pick_dest_folder");
    if (picked) {
      state.dest = picked;
      renderDestSummary();
    }
  });

  el("convert-btn").addEventListener("click", runConvert);

  await listen("convert-progress", (event) => {
    const { index, total, name, status } = event.payload;
    const pct = Math.round(((index - 1 + (status === "running" ? 0 : 1)) / total) * 100);
    el("progress-bar-fill").style.width = `${pct}%`;
    el("progress-label").textContent =
      status === "running"
        ? `Converting ${index} of ${total}: ${name}`
        : `${status === "done" ? "Done" : "Error"}: ${name} (${index} of ${total})`;
  });
}

async function runConvert() {
  const settings = await invoke("get_settings");

  el("convert-btn").disabled = true;
  el("pick-files").disabled = true;
  el("pick-dest").disabled = true;
  el("result-area").hidden = true;
  el("progress-area").hidden = false;
  el("progress-bar-fill").style.width = "0%";
  el("progress-label").textContent = "Starting…";

  const summary = await invoke("convert", {
    files: state.files,
    dest: state.dest,
    threadCount: settings.thread_count,
    trimNone: settings.trim_none,
  });

  el("progress-area").hidden = true;
  el("convert-btn").disabled = false;
  el("pick-files").disabled = false;
  el("pick-dest").disabled = false;

  const resultArea = el("result-area");
  resultArea.hidden = false;
  if (summary.errors.length === 0) {
    resultArea.className = "success";
    resultArea.textContent = `Converted ${summary.success_count} of ${summary.total_count} file(s) successfully.`;
  } else {
    resultArea.className = "error";
    resultArea.textContent =
      `Converted ${summary.success_count} of ${summary.total_count} file(s).\n\nErrors:\n` +
      summary.errors.join("\n");
  }
}

init();
