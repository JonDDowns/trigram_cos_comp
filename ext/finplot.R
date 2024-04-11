library(ggplot2)
dat <- data.table::fread("./out/results.csv")
lablevs <- c("R baseline", "Rust (v1)", "Rust (v2)", "Rust (final)")
dat$Label <- ordered(dat$Label, levels = lablevs)
p <- ggplot(dat, aes(x = Label, y = CalcsPerSec / 1000)) +
        geom_bar(stat = "identity") +
        scale_y_continuous(labels = scales::comma_format(), limits = c(0, 90000)) +
        theme_minimal() +
        ylab("Comparisons per Second (in thousands)") +
        xlab("Version") +
        ggtitle("Cosine Trigram Comparisons per Second by Method") +
        theme(
                axis.text = element_text(size = 12, color = "black"),
                axis.title = element_text(size = 16),
                plot.title = element_text(size = 22, color = "black")
        )
ggsave("./out/cos_tri_benchmarks.svg", plot = p, width = 10, height = 8)
