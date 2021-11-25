# kitkat clock in Rust

This is the plan9 cat clock utility rewritten in rust with [minifb](https://crates.io/crates/minifb) crate.

```shell
$ kitkat --help
Usage: kitkat [--hook|--crazy|--offset OFFSET|--borderless|--resize|--sunmoon|--moon|--date]

Displays a kit kat clock with the system time, or the system time with given offset if the --offset
argument is provided.

      --hook                 show a hooked tail instead of the default drop shaped one
      --crazy                go faster for each time this argument is invoked
      --offset OFFSET        add OFFSET to current system time (only the first given
                             offset will be used)
      --borderless
      --resize
      --sunmoon              show sun or moon phase depending on the hour
      --moon                 show only moon phase
      --date                 show month date

      OFFSET format is [+-]{0,1}\d\d:\d\d, e.g: 02:00 or -03:45 or +00:00
```

## Demo

<table>
<tr>
<td>

![demo](./kitkat-round.gif?raw=true)

</td>
<td>

![demo](./kitkat-hook.gif?raw=true)

</td>
<td>

![demo](./kitkat-resized.jpg?raw=true)

</td>
</tr>
</table>

## References

- https://github.com/BarkyTheDog/catclock
