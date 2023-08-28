# @nwsseveretstorm.bsky.social
This is a rough mirror of the [NWS Severe Tstorm Twitter bot](https://twitter.com/nwsseveretstorm) but it's built to be distributed to multiple platforms. It's purely a tool for social media platforms and should not be relied upon for safety. Please see the official [NWS website](https://weather.gov) for time- (and life-) critical information.

## Check it out
You can check it out here: [@nwsseveretstorm.bsky.social](https://bsky.app/profile/nwsseveretstorm.bsky.social).

## How to run
1. Clone the repo:
```
git clone https://git.sr.ht/~jordanreger/nwsseveretstorm
```

2. Add a `.env` in the `src` directory:
```
touch src/.env
```

3. Add a file called `warning` in the `src` directory:
```
touch src/warning
```

4. Run it:
```
cargo run
```