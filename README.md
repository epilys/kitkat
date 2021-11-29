# kitkat clock in Rust

This is the plan9 cat clock utility rewritten in rust with [minifb](https://crates.io/crates/minifb) crate.

Confirmed working in Linux and Windows.

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
      --dog                  show an italian greyhound named Gaius Octavius Maximus instead of a cat

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
<td>

![demo](./dogkat.gif?raw=true)

</td>
</tr>
<tr>
<th>
Default drop shaped tail.
</th>
<th>
Hooked tail with `--hook`.
</th>
<th>
Resizable window with `--resize`.
</th>
<th>
Dog instead of cat with `--dog`.
</th>
</tr>
<tr>
<td>

![demo](./kitkat-date-and-sun.jpg?raw=true)

</td>
<td>

![demo](./kitkat-moon-phase-only.jpg?raw=true)

</td>
<td>
</td>
<td>
</td>
</tr>
<tr>
<th>
Showing current date and sun/moon phase status.
</th>
<th>
Showing just moon phase status.
</th>
</tr>
</table>

## References

- https://github.com/BarkyTheDog/catclock
