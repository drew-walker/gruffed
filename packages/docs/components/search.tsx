"use client";

import { Index } from "flexsearch";
import { SearchIcon } from "lucide-react";
import Link from "next/link";
import { useMemo, useState } from "react";
import { searchItems } from "../lib/nav";
import styles from "./search.module.css";

export function Search() {
  const [query, setQuery] = useState("");
  const index = useMemo(() => {
    const nextIndex = new Index({ tokenize: "forward" });
    searchItems.forEach((item, id) => {
      nextIndex.add(id, `${item.title} ${item.summary} ${item.content}`);
    });
    return nextIndex;
  }, []);

  const results = useMemo(() => {
    const trimmed = query.trim();
    if (trimmed.length < 2) {
      return [];
    }
    return index
      .search(trimmed, { limit: 6 })
      .map((id) => searchItems[Number(id)])
      .filter((item): item is (typeof searchItems)[number] => Boolean(item));
  }, [index, query]);

  return (
    <div className={styles.search}>
      <label className={styles.inputWrap}>
        <SearchIcon size={15} aria-hidden="true" />
        <input
          className={styles.input}
          value={query}
          onChange={(event) => setQuery(event.target.value)}
          placeholder="Search docs"
          type="search"
        />
      </label>
      {results.length > 0 ? (
        <div className={styles.results}>
          {results.map((item) => (
            <Link className={styles.result} href={item.href} key={item.href}>
              <span className={styles.resultTitle}>{item.title}</span>
              <span className={styles.resultMeta}>{item.group} / {item.summary}</span>
            </Link>
          ))}
        </div>
      ) : null}
    </div>
  );
}
