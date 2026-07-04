"use client";

import {
  Bar,
  BarChart,
  CartesianGrid,
  Legend,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts";
import styles from "./benchmark-chart.module.css";

export type BenchmarkPoint = {
  name: string;
  files: number;
  sequentialMs: number;
  parallelMs: number;
  memoryMb: number;
};

export function BenchmarkChart({ data }: Readonly<{ data: BenchmarkPoint[] }>) {
  return (
    <div className={styles.wrap}>
      <ResponsiveContainer width="100%" height="100%">
        <BarChart data={data} margin={{ top: 8, right: 18, left: 0, bottom: 8 }}>
          <CartesianGrid stroke="#e5edf3" vertical={false} />
          <XAxis dataKey="name" tick={{ fill: "#536273", fontSize: 12 }} tickLine={false} />
          <YAxis tick={{ fill: "#536273", fontSize: 12 }} tickLine={false} />
          <Tooltip
            cursor={{ fill: "rgba(8, 115, 111, 0.08)" }}
            contentStyle={{
              border: "1px solid #d9e2ec",
              borderRadius: 8,
              boxShadow: "0 18px 46px rgba(24, 33, 47, 0.12)",
            }}
          />
          <Legend />
          <Bar dataKey="sequentialMs" name="Sequential ms" fill="#9aa9ba" radius={[5, 5, 0, 0]} />
          <Bar dataKey="parallelMs" name="Parallel ms" fill="#08736f" radius={[5, 5, 0, 0]} />
        </BarChart>
      </ResponsiveContainer>
    </div>
  );
}
