import { create } from "zustand";
import { invoke } from "@tauri-apps/api/tauri";

export type SearchResult = {
  id: number;
  embedding: Array<number>;
  ocr_text: String;
  timestamp: number;
  focused_window_title: String;
};

export type SearchResults = Array<SearchResult>;

export const useTimelineStore = create<{
  timeline: SearchResults;
  setTimeline: (v: SearchResults) => void;
  addTimeline: (v: SearchResults) => void;
}>()((set) => ({
  timeline: [],
  setTimeline: (v) =>
    set((state) => ({
      timeline: v
        .filter(
          (value, index, self) =>
            self.findIndex((t) => t.id === value.id) === index,
        )
        .sort((a, b) => a.timestamp - b.timestamp),
    })),
  addTimeline: (v) =>
    set((state) => ({
      timeline: state.timeline
        .concat(v)
        .sort((a, b) => a.timestamp - b.timestamp)
        // dedupe by id
        .filter(
          (value, index, self) =>
            self.findIndex((t) => t.id === value.id) === index,
        ),
    })),
}));

export function fetchTimelineSegment(
  args: { after?: number; before?: number; limit?: number } = { limit: 10 },
) {
  invoke("get_rows", args).then((result) => {
    useTimelineStore.getState().addTimeline(result as SearchResults);
  });
}

export function fetchMoreTimeline() {
  const currentTimeline = useTimelineStore.getState().timeline;

  if (currentTimeline.length === 0) {
    fetchTimelineSegment();
  } else {
    const firstTimestamp = currentTimeline[0].timestamp;
    fetchTimelineSegment({ before: firstTimestamp });
  }
}

export function useTimeline() {
  useEffect(() => {
    fetchTimelineSegment();
  }, []);
  return [
    useTimelineStore((state) => state.timeline),
    fetchMoreTimeline,
  ] as const;
}
