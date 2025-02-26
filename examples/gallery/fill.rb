#!/usr/bin/env ruby

gallery = "| | | |
|:-------------------------:|:-------------------------:|:-------------------------:|
"

Dir.glob("*.svg").each_with_index do |file, i|
  if file == "test.svg" then next end

  title = file
    .sub(/^gallery\//, "")
    .sub(/\.svg$/, "")
    .gsub(/-/, " ").split(" ")
    .map { |word| word.upcase == word ? word : word.capitalize }
    .join(" ")

  gallery += "| **#{title}** ![#{title}](./examples/gallery/#{file})"
  if i % 3 == 2 then gallery += "\n" end
end

File.open "../../README.md", "w" do |f|
  f.write File.read("../../README.md.in").gsub("%gallery%", gallery)
end
