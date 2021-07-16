# HoursTracker timesheet reporter

Takes in a CSV file that was exported by the [HoursTracker](http://www.hourstrackerapp.com/) app and produces and
console report displaying:

- Work times for each day (grouped by week)
- Summary of total work time, expected work time and resulting overtime

It's currently hacked together for my own needs. I work regularly 8 hours a day (salaried) and any minute I work over
that, is overtime that I can use later on to leave early.

## Workdays

A day is only counted as a workday if you worked more than a configurable amount on that day. This is for day were you
actually wasn't expected to work but did anyway (e.g. your replied to an urgent mail during a day off). Those days still
count towards the total time worked but don't add to the expected time.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
