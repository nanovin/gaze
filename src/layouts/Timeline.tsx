import { useTimeline } from "../lib/timeline.ts";

export function Timeline() {
  const [timeline, fetchMoreTimeline] = useTimeline();
  return <div></div>;
}
