# Die Kuckucksuhr

[Kuckuck!](https://twitter.com/die_kuckucksuhr)

## Kompilieren

    % cargo build --release

## Konfigurieren

    % mv kuckuck.toml-dist kuckuck.toml
    % ed kuckuck.toml

(Ihr braucht einen Twitteraccount. Die Token kriegt ihr [hier](https://developer.twitter.com/en/apps).)

## Laufen lassen

    % crontab -e
    # ...
    0 * * * * /home/kuckucksuhr/kuckucksuhr
