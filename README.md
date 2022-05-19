# Thesaurus web lib

This is a rust library that scrapes synonym/antonym/etc data from Thesaurus.com. If you are from thesaurus.com, kindly avert your eyes. Nothing is going on here.

This was mostly a personal library that was intented for my own use but I am publishing it to make a few other projects more available.

I removed a lot of parts from the JSON struct that panicked when it didn't exist. Ideally I would simply remove every part of the struct that isn't used, but the usage is all tied together in different projects. 

I have scraped every single word on Thesaurus.com with this library and it didn't panic once. (I picked one word like "good", added every synonym/antonym to a vec, then did the same for every word in that vec)

If you have any bug reports, please let me know by creating an issue. But, no guarantees for stability! I wrote this not thinking it would be published.