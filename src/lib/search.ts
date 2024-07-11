import { SearchResults } from "./timeline.ts";
import { create } from "zustand";
import { invoke, convertFileSrc } from "@tauri-apps/api/tauri";
import { appDataDir, join } from "@tauri-apps/api/path";

export const useSearchResultsStore = create<{
  results: SearchResults;
  setResults: (v: SearchResults) => void;
  addResults: (v: SearchResults) => void;
}>()((set) => ({
  results: [],
  setResults: (v) =>
    set((state) => ({
      results: v
        .filter(
          (value, index, self) =>
            self.findIndex((t) => t.id === value.id) === index,
        )
        .sort((a, b) => a.timestamp - b.timestamp),
    })),
  addResults: (v) =>
    set((state) => ({
      results: state.results
        .concat(v)
        .sort((a, b) => a.timestamp - b.timestamp)
        // dedupe by id
        .filter(
          (value, index, self) =>
            self.findIndex((t) => t.id === value.id) === index,
        ),
    })),
}));

export async function vectorSearch(q: String, limit = 10) {
  if (!q) {
    return;
  }

  invoke("vector_search", { query: q, limit: limit }).then(
    async (results: SearchResults) => {
      const appDataDirPath = await appDataDir();

      results = await Promise.all(
        results.map(async (result) => {
          const filePath = await join(
            appDataDirPath,
            `/screenshots/${result.id}.png`,
          );
          result.url = convertFileSrc(filePath);
          return result;
        }),
      );

      useSearchResultsStore.getState().setResults(results);
    },
  );
}
