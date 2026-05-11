FROM ghcr.io/foundry-rs/foundry:nightly AS builder

WORKDIR /app

COPY lib/ lib/
COPY src/ src/
COPY test/ test/
COPY script/ script/
COPY foundry.toml remappings.txt ./

RUN forge install --no-commit && \
    forge build --force

FROM ghcr.io/foundry-rs/foundry:nightly

WORKDIR /app

COPY --from=builder /app/out/ out/
COPY --from=builder /app/broadcast/ broadcast/
COPY --from=builder /app/lib/ lib/
COPY script/ script/
COPY foundry.toml remappings.txt .env* ./

ENTRYPOINT ["forge"]
CMD ["--help"]
