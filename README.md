# Die Kuckucksuhr

[Kuckuck!](https://twitter.com/kuckucksuhr)

## Kompilieren

    % cargo build --release

## Konfigurieren

    % ed kuckuck.toml

(Ihr braucht einen Twitteraccount. Die Token kriegt ihr [hier](https://dev.twitter.com).)

## Laufen lassen

    % crontab -e
    # ...
    0 * * * * /home/kuckucksuhr/kuckucksuhr
