import { SearchInput } from "../components/SearchInput";
import { useSearchResultsStore } from "../lib/search.ts";

function Result({ result }) {
  return (
    <div className="flex flex-col gap-1.5">
      <div className="w-full h-32 rounded bg-black/10" />
      <h1 className="text-xs truncate opacity-25">
        {result.focused_window_title}
      </h1>
      {/* <p>{result.ocr_text}</p> */}
    </div>
  );
}

function Results() {
  const searchResults = useSearchResultsStore((s) => s.results);

  if (searchResults.length <= 0) {
    return (
      <div className="flex justify-center items-center h-full w-full">
        <div className="flex flex-col gap-2 items-center">
          <p className="opacity-25">Search for something you saw...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="p-4 grid grid-cols-3 gap-4 overflow-y-auto overflow-x-hidden">
      {searchResults.map((result) => (
        <Result key={result.id} result={result} />
      ))}
    </div>
  );
}

export function Search() {
  return (
    <div className="w-full h-full flex flex-col">
      <SearchInput />
      <Results />
    </div>
  );
}
