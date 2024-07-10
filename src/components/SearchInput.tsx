import { vectorSearch } from "../lib/search.ts";

export function SearchInput() {
  const timeout = useRef(null);
  const [q, setQ] = useState("");

  useEffect(() => {
    if (timeout.current) {
      clearTimeout(timeout.current);
    }
    timeout.current = setTimeout(() => {
      vectorSearch(q);
    }, 500);
  }, [q]);

  return (
    <input
      type="text"
      value={q}
      autoFocus
      placeholder="Search for something you saw..."
      className="w-full appearance-none border-b text-lg border-white/10  p-4 bg-transparent focus:outline-none"
      onChange={(e) => setQ(e.target.value)}
      onKeyPress={(e) => {
        if (e.key === "Enter") {
          vectorSearch(q);
          e.target.blur();
          if (timeout.current) {
            clearTimeout(timeout.current);
          }
        }
      }}
    />
  );
}
