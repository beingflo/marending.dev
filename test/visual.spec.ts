import { test, expect } from "@playwright/test";

const notes = [
  "/",
  "/projects/",
  "/about/",
  "/notes/launchd/",
  "/notes/jour/",
  "/notes/reflective/",
  "/notes/monitoring/",
  "/notes/otel/",
  "/notes/llm/",
  "/notes/clickhouse/",
  "/notes/server-setup/",
  "/notes/jpeg-raw/",
  "/notes/go-rest-quest/",
  "/notes/responsive-plots/",
  "/notes/co2-loft/",
  "/notes/sqlite-vs-duckdb/",
  "/notes/unstructured-data/",
  "/notes/rest-quest/",
  "/notes/fieldnotes/",
  "/notes/rust-spa/",
  "/notes/16-hour-projects/",
  "/notes/wasm-benchmark/",
  "/notes/vps-benchmarks/",
  "/notes/sqlite-benchmarks/",
  "/notes/league-rating/",
  "/notes/league-data/",
  "/notes/co2-bedroom/",
  "/notes/esp-protocol/",
  "/notes/esp-power/",
  "/notes/performance/",
  "/notes/website/",
  "/notes/visual-testing/",
  "/notes/passkeys/",
  "/feedback/",
];

test.describe("Visual regression", () => {
  const baseUrl = "https://marending.dev";

  for (const note of notes) {
    test(`capture page: -${note}-`, async ({ page }) => {
      const url = `${baseUrl}${note}`;

      await page.goto(url);
      await page.waitForTimeout(200);

      const pageHeight = await page.evaluate(() => document.body.scrollHeight);

      for (let scrolled = 0; scrolled < pageHeight; scrolled += 200) {
        await page.mouse.wheel(0, 200);
        await page.waitForTimeout(200);
      }

      await page.waitForLoadState("networkidle");

      const screenshotName =
        note
          .replace(/^\//, "")
          .replace(/\/$/, "")
          .replace(/[^a-z0-9]/gi, "-")
          .toLowerCase() || "index";

      await expect(page).toHaveScreenshot(`${screenshotName}.png`, {
        fullPage: true,
      });
    });
  }
});
