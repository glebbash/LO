import https from "node:https";

import express from "express";
import cors from "cors";

const app = express();
app.use((req, res, next) => {
    // fix `.png` and `.svg` assets
    res.setHeader("Cross-Origin-Resource-Policy", "cross-origin");

    const startTime = Date.now();
    res.on("finish", function () {
        const now = new Date().toISOString();
        const duration = Date.now() - startTime;
        console.log(
            `[${now}] ${req.method} ${req.url} - ${this.statusCode} [${duration}ms]`
        );
    });
    next();
});
app.use(cors());
app.use(express.static("."));

https.createServer(getCredentials(), app).listen(5000, () => {
    console.log("serving on https://localhost:5000");
});

// localhost certificates
function getCredentials() {
    return {
        key: `-----BEGIN PRIVATE KEY-----
MIIEvwIBADANBgkqhkiG9w0BAQEFAASCBKkwggSlAgEAAoIBAQDwkKCjyp2qDe3E
DhWpnTfOHnZZ9R41UujE96UfXUUlnd9AxM5VHiK6azr8+mz1QJ470JOg+mw8qLxH
1E5mfy3kFmjPxhI72u3MCIwMK26706WIWyVaNW1LVmfRDgwHYigMCvFa+ZknErbe
fIEdVggdYviHLm+VPJKh6m1rLijI1aQ6MvNmKvoQ8ARrvVWDDZ2WYeyON7iP7ZKT
Tb/wN0ZIF+MhgRtYr0ez5J3Mj43HbV0ayeLfHFOkjdj2dwq7LN5AhIUW4f4Q84WW
MdvWz0YRAje5uNKT0BFWz2m7vsaJeVopFCyUa7suFTK1NOCFnfywR9tMgUET8ngu
otdMfuYdAgMBAAECggEBAI3BC4e4qzM7xfGTQxyRcgLZXHAhyk0tr8/xBbH3qflC
j/2Aev0ZJummqjnL70yNaTBh9bwUFJeh7mB9MD2hj48pzVUxqCjxFtL17mB1b2jq
jE5PNzFy7hZIsiA0IM5w/pwqDpLHJYIquutbflGlEl8OpNgzerj72oPiudtAdfPx
G7u2sVFpnp4rQTemZYoKyX7MCZtVH6Xw3NBhYb8KWV5TdSb/nmE1c0vRAthSOyHD
diTlrZfDwsDPYNrv2Y71BW7afXCsQ4gZelGT5H/JVdgEVq8XF4k+ytacIIylAn3G
szMgsuWocVM4v4iTd1PkhcarG7avQa0Cixgyx1UYkRkCgYEA+qacbHCZivBuiL5a
gQvuq+GxKq6INrEzlCR/1dA7TJHH+NO5LSsHt2s2drxsGZ1xkID547GPUqF6HUn6
MTzeFgI9liYw0lbox+dvATOOArSozFt4GKJfj+AdOw/r2hZU0KVf8NA9WDaGdvky
F9vxVHbKfHnOnJeCuPeE24W5B6MCgYEA9bLp+tjpRMFu3A6z6/FD6STUjSnz0dKG
llyt5T5AdsH8rz/Xda789JlrfJT9jXXea4J3WaDOtR57Csu7dtJOmBB1LPDCpMHY
IdO8iu6uYSSEs8z/uyyD5ta7JnxgUDmKsoXW2aC0/T13g3H1PBd98374KgEYOTM1
9E8WqwyGNz8CgYA60HGP+HPu6C/nolL2SOh+KH07+Kw5uSbTMiVU9IGVm/eCaZvf
/2LWJTAoeFodQJZvQ/Re+0EYMI/I45zzhMbP/0KYQund0UrZ7XJco85E0ENzgFsT
3LXzurDhNQEwsWDBfKyggxyB4Hl43Uxl0NQ175Hf120lzV99K4dfQ2eBZQKBgQDk
sIsoeQ1mecT9/pFpYz+amjVcz1eqTRIIkmspnS/TWewgEoQxfZbk9IgfhW4bZUKJ
Pv5n5k2AN8a+3bK3fji5QVhPZClHwUWy9qnfhxm4QY1i7cR6K4z6J5Q61jmvcl+u
mvTHy9WqgLPlHQTt0c/h/KRwNGSfmRL3O0YWH6otcQKBgQDVvL20wpost13+XOyz
SQOqU+b190oSup/pLmZqk55VrmSuILjPApaLeJKtVjqjf6AwesCWIEk8ykRlM5Yb
LTUjLPAgwaNc3JQ4wqEKQ8ZncEe3L9coBHZ3X/62B9nU+8/DuPA5b6Clk7PYxnE7
P+ev5Jndsc7YsvBzsIDDWGxQpg==
-----END PRIVATE KEY-----`,
        cert: `-----BEGIN CERTIFICATE-----
MIIEUzCCArugAwIBAgIRANMUxSDNrJrn7xaxd8Tv8V8wDQYJKoZIhvcNAQELBQAw
gY0xHjAcBgNVBAoTFW1rY2VydCBkZXZlbG9wbWVudCBDQTExMC8GA1UECwwoR0xF
QkJBU0hcZ2xlYmJAZ2xlYmJhc2ggKEdsZWIgQmFzaGthdG92KTE4MDYGA1UEAwwv
bWtjZXJ0IEdMRUJCQVNIXGdsZWJiQGdsZWJiYXNoIChHbGViIEJhc2hrYXRvdikw
HhcNMjQwNDI0MTkyMTEyWhcNMjYwNzI0MTkyMTEyWjBcMScwJQYDVQQKEx5ta2Nl
cnQgZGV2ZWxvcG1lbnQgY2VydGlmaWNhdGUxMTAvBgNVBAsMKEdMRUJCQVNIXGds
ZWJiQGdsZWJiYXNoIChHbGViIEJhc2hrYXRvdikwggEiMA0GCSqGSIb3DQEBAQUA
A4IBDwAwggEKAoIBAQDwkKCjyp2qDe3EDhWpnTfOHnZZ9R41UujE96UfXUUlnd9A
xM5VHiK6azr8+mz1QJ470JOg+mw8qLxH1E5mfy3kFmjPxhI72u3MCIwMK26706WI
WyVaNW1LVmfRDgwHYigMCvFa+ZknErbefIEdVggdYviHLm+VPJKh6m1rLijI1aQ6
MvNmKvoQ8ARrvVWDDZ2WYeyON7iP7ZKTTb/wN0ZIF+MhgRtYr0ez5J3Mj43HbV0a
yeLfHFOkjdj2dwq7LN5AhIUW4f4Q84WWMdvWz0YRAje5uNKT0BFWz2m7vsaJeVop
FCyUa7suFTK1NOCFnfywR9tMgUET8nguotdMfuYdAgMBAAGjXjBcMA4GA1UdDwEB
/wQEAwIFoDATBgNVHSUEDDAKBggrBgEFBQcDATAfBgNVHSMEGDAWgBQHA+9WgcDl
Q7rWftxx2OsfSn6KRDAUBgNVHREEDTALgglsb2NhbGhvc3QwDQYJKoZIhvcNAQEL
BQADggGBAHyETvz9+OOtPkWt+IhKDuukNtX93U9o7fHYAt0MtStiT08ISDBo1Ke3
P6jGQfijdGktk5JgdVRiDwhOzGLwvUxujE/rmNg+wuOOwofN50B4aPLDjOmQhYQl
v5eRtEo+obKZqmp3cS/FI5GmZq4KECo2NHwST5VqcmsaO4lxfyfWH0jfeeDl3OUM
TtwqxED315/Ehs4K6HNsSobZT/H1RYREYsY8+WajN/XQsdEeTaJIYV3RGUTBHWLr
uSB5Md6E7IWBlhd68YthlHnEEn6y06mLXS4A+LekSnhDH113X7nqaQQhj6Xa5oTc
7YcbcgAvIBAvCTeKo3SttJjLHwaq/BFNRO5UuH+uio0VSto4t4xIFU5907M04bVi
PXRoefEOnFHyOSn0G+V+5/FxNDrxc3dBtOMwxcQ1THz3yjJnmVBij112SxPpK2QC
NW8gKRLwGPgD3tbO4+fhL+p8ox/T596xqS5R8sFZKjrWfklO8yM2yCipHCzARCTt
7WegTMkvhw==
-----END CERTIFICATE-----`,
    };
}
