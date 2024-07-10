import { SearchResults } from "./timeline.ts";
import { create } from "zustand";
import { invoke } from "@tauri-apps/api/tauri";

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

  invoke("vector_search", { query: q, limit: limit }).then((result) => {
    useSearchResultsStore.getState().setResults(result as SearchResults);
  });
}
