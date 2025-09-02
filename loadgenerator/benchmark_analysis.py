#!/usr/bin/env python3

import numpy as np
import re
import json
SPANS = ["DB_Store RPC", "LLM RPC", "Ads RPC", "DB_Read RPC"]


def extract_bench_line(line):
    match = re.search(r"(elapsed=)([0-9]*[.]?[0-9]+)", line)
    if match:
        val = float(match.group(2))
        unit = "ms" if "m" in line[match.span()[1]:] else "us"
        if unit == "ms":   
            val = val * 1000.0
        elif unit == "us":
            val = float(val)
        return val



class BenchmarkResult():
    def __init__(self):
        self.values = []

    def median(self):
        if len(self.values) >0:
            return np.median(self.values)

    def tail_lat(self):
        if len(self.values) > 0:
            return np.percentile(self.values, 95)
    
    def append(self, value):
        self.values.append(value)


with open("benchmark.log") as bench_file:
    bench_store = dict([(span, BenchmarkResult()) for span in SPANS])
    for line in bench_file:
        for span in SPANS:
            if span in line:
                micro_val = extract_bench_line(line)
                if micro_val is not None:
                    bench_store[span].append(micro_val)


    bench_results = [(span, bench.median(), bench.tail_lat()) for (span, bench) in bench_store.items()]
    print(bench_results)
    with open("results.json", 'a+') as dump_file:
        json.dump(bench_results, dump_file)
        dump_file.write(\n)
