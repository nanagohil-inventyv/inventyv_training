let n = 5;
for (let i = 1; i <= n; i++) {
    for (let j = 1; j <= n; j++) {
        let di = -(~(n - i));
        let dj = -(~(n - j));
        // min4 using ^ and & (no ~ here)
        let min_ij = j ^ ((i ^ j) & -(i < j));
        let min_d = dj ^ ((di ^ dj) & -(di < dj));
        let val = min_d ^ ((min_ij ^ min_d) & -(min_ij < min_d));
        process.stdout.write(val + " ");
    }
    console.log("");
}