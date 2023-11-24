

# Template 

# Date formatting:
For formatting strign see: https://time-rs.github.io/book/api/format-description.html

Date can parse the following formats:
- Iso8601
- Rfc3339
- Rfc2822
- [year]-[month]-[day] [hour]:[minute]:[second]
- [year]-[month]-[day] [hour]:[minute]
- [year]-[month]-[day] [hour]

Usage:
```
Format with default format: {{date now}} 
Format with custom formatter: {{date now fmt="[day padding:zero]/[month padding:zero]/[year]"
Format with custom formatter alternative: {{date now fmt="[day]/[month]/[year repr:last_two]"}}
```
