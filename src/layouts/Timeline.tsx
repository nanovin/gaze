import { useTimeline } from "../lib/timeline.ts";

export function Timeline() {
  const [timeline, fetchMoreTimeline] = useTimeline();
  console.log(timeline);
  return (
    <div>
      <h1>Timeline</h1>
      {timeline.map((screenshot) => (
        <div key={screenshot.id}>
          <p>{screenshot.ocr_text}</p>
        </div>
      ))}
    </div>
  );
}
