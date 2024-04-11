# trigram_cos_comp

This is sharing the code for some exploratory work I did comparing R and Rust performance on
a string distance calculation.
Please see the relevant post on [my website]() for a deeper explanation of the code and its findings.

Most of the project is organized as a traditional Rust project. 
All Rust code is in the src subfolder.
The R code, shell scripts for benchmarking, and data files are in the ext folder.
The test data were borrowed from [here](https://github.com/dominictarr/random-name/blob/master/first-names.txt).
Version 1 of the Rust function uses the [textdistance](https://docs.rs/textdistance/latest/textdistance/) crate.
For versions 2 and 3, much of the textdistance logic is preserved, but was pulled directly into the codebase
to allow for more performance optimization.
Again, the related blog post on my website discusses this and more in greater detail.
