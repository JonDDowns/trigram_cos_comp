library(stringdist)
library(Matrix)
library(data.table)

# Load data, specify number of rows
nrows <- 10000
print(paste("Reading in", nrows, "rows of data"))
df <- data.table::fread(
                          "./ext/example.csv",
                          col.names = c("id"),
                          nThread = 16,
                          header = FALSE,
                          nrows = nrows
)

print("Begin stringdist calculation...")
strmat <- stringdistmatrix(df$id, q=3, method="cosine", useNames="strings", nthread=16)

# The following steps remove duplicated calculations and cases where 
# identical strings are compared
print("Tidying up data...")
strmat <- as.matrix(strmat)
strmat[lower.tri(strmat, diag=TRUE)] <- NA # Uniquely mark duplicate rows & self-compares
df <- data.table(as.data.frame.table(strmat)) # Convert to DT
df <- df[!(!is.nan(Freq) & is.na(Freq)), ] # Filter unneeded data

print(paste("Number of calculations performed:", nrow(df)))
