import rottnest

# rottnest.index_file_bm25("example_data/0.parquet", "body", "index0")
# rottnest.index_file_bm25("example_data/1.parquet", "body", "index1")
# rottnest.merge_index_bm25("merged_index", ["index0", "index1"])
# result = rottnest.search_index_bm25(["merged_index"], "cell phones", K = 10,query_expansion = "openai", reader_type = "aws")
# print(result)

# rottnest.index_file_substring("example_data/real.parquet", "text", "index0", token_skip_factor = 3)
# rottnest.index_file_substring("example_data/1.parquet", "body", "index1")
# rottnest.merge_index_substring("merged_index", ["index0", "index1"])
# result = rottnest.search_index_substring(["index0"], "did fake", K = 10)
# print(result)

# rottnest.index_file_uuid("a.parquet", "hashes", "index0")
# rottnest.index_file_uuid("b.parquet", "hashes", "index1")
rottnest.merge_index_uuid("merged_index", ["index0", "index1"])
result = rottnest.search_index_uuid(["index1"], "93b9f88dd22cb168cbc45000fcb05042cd1fc4b5602a56e70383fa26d33d21b08d004d78a7c97a463331da2da64e88f5546367e16e5fd2539bb9b8796ffffc7f", K = 10)
print(result)
result = rottnest.search_index_uuid(["merged_index"], "650243a9024fe6595fa953e309c722c225cb2fae1f70c74364917eb901bcdce1f9a878d22345a8576a201646b6da815ebd6397cfd313447ee3a548259f63825a", K = 10)
print(result)
result = rottnest.search_index_uuid(["merged_index"], "32b8fd4d808300b97b2dff451cba4185faee842a1248c84c1ab544632957eb8904dccb5880f0d4a9a7317c3a4490b0222e4deb5047abc1788665a46176009a07", K = 10)
print(result)