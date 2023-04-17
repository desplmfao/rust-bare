(() => {
    "use strict";
    const e = self.Ultraviolet,
        t = ["cross-origin-embedder-policy", "cross-origin-opener-policy", "cross-origin-resource-policy", "content-security-policy", "content-security-policy-report-only", "expect-ct", "feature-policy", "origin-isolation", "strict-transport-security", "upgrade-insecure-requests", "x-content-type-options", "x-download-options", "x-frame-options", "x-permitted-cross-domain-policies", "x-powered-by", "x-xss-protection"],
        r = ["GET", "HEAD"];
    class i extends e.EventEmitter {
        constructor(t = __uv$config) {
            super(), t.bare || (t.bare = "/bare/"), t.prefix || (t.prefix = "/service/"), this.config = t;
            const r = (Array.isArray(t.bare) ? t.bare : [t.bare]).map((e => new URL(e, location).toString()));
            this.address = r[~~(Math.random() * r.length)], this.bareClient = new e.BareClient(this.address)
        }
        async fetch({
            request: i
        }) {
            let a;
            try {
                if (!i.url.startsWith(location.origin + this.config.prefix)) return await fetch(i);
                const c = new e(this.config, this.address);
                "function" == typeof this.config.construct && this.config.construct(c, "service");
                const l = await c.cookie.db();
                c.meta.origin = location.origin, c.meta.base = c.meta.url = new URL(c.sourceUrl(i.url));
                const d = new o(i, this, c, r.includes(i.method.toUpperCase()) ? null : await i.blob());
                if ("blob:" === c.meta.url.protocol && (d.blob = !0, d.base = d.url = new URL(d.url.pathname)), i.referrer && i.referrer.startsWith(location.origin)) {
                    const e = new URL(c.sourceUrl(i.referrer));
                    (d.headers.origin || c.meta.url.origin !== e.origin && "cors" === i.mode) && (d.headers.origin = e.origin), d.headers.referer = e.href
                }
                const h = await c.cookie.getCookies(l) || [],
                    u = c.cookie.serialize(h, c.meta, !1);
                d.headers["user-agent"] = navigator.userAgent, u && (d.headers.cookie = u);
                const p = new n(d, null, null);
                if (this.emit("request", p), p.intercepted) return p.returnValue;
                a = d.blob ? "blob:" + location.origin + d.url.pathname : d.url;
                const m = await this.bareClient.fetch(a, {
                        headers: d.headers,
                        method: d.method,
                        body: d.body,
                        credentials: d.credentials,
                        mode: location.origin !== d.address.origin ? "cors" : d.mode,
                        cache: d.cache,
                        redirect: d.redirect
                    }),
                    f = new s(d, m),
                    b = new n(f, null, null);
                if (this.emit("beforemod", b), b.intercepted) return b.returnValue;
                for (const e of t) f.headers[e] && delete f.headers[e];
                if (f.headers.location && (f.headers.location = c.rewriteUrl(f.headers.location)), "document" === i.destination) {
                    const e = f.headers["content-disposition"];
                    if (!/\s*?((inline|attachment);\s*?)filename=/i.test(e)) {
                        const t = /^\s*?attachment/i.test(e) ? "attachment" : "inline",
                            [r] = new URL(m.finalURL).pathname.split("/").slice(-1);
                        f.headers["content-disposition"] = `${t}; filename=${JSON.stringify(r)}`
                    }
                }
                if (f.headers["set-cookie"] && (Promise.resolve(c.cookie.setCookies(f.headers["set-cookie"], l, c.meta)).then((() => {
                        self.clients.matchAll().then((function(e) {
                            e.forEach((function(e) {
                                e.postMessage({
                                    msg: "updateCookies",
                                    url: c.meta.url.href
                                })
                            }))
                        }))
                    })), delete f.headers["set-cookie"]), f.body) switch (i.destination) {
                    case "script":
                    case "worker": {
                        const e = [c.bundleScript, c.clientScript, c.configScript, c.handlerScript].map((e => JSON.stringify(e))).join(",");
                        f.body = `if (!self.__uv && self.importScripts) { ${c.createJsInject(this.address, this.bareClient.data, c.cookie.serialize(h, c.meta, !0), i.referrer)} importScripts(${e}); }\n`, f.body += c.js.rewrite(await m.text())
                    }
                    break;
                    case "style":
                        f.body = c.rewriteCSS(await m.text());
                        break;
                    case "iframe":
                    case "document":
                        (function(t, r = "") {
                            return "text/html" === (e.mime.contentType(r || t.pathname) || "text/html").split(";")[0]
                        })(c.meta.url, f.headers["content-type"] || "") && (f.body = c.rewriteHtml(await m.text(), {
                            document: !0,
                            injectHead: c.createHtmlInject(c.handlerScript, c.bundleScript, c.clientScript, c.configScript, this.address, this.bareClient.data, c.cookie.serialize(h, c.meta, !0), i.referrer)
                        }))
                }
                return "text/event-stream" === d.headers.accept && (f.headers["content-type"] = "text/event-stream"), this.emit("response", b), b.intercepted ? b.returnValue : new Response(f.body, {
                    headers: f.headers,
                    status: f.status,
                    statusText: f.statusText
                })
            } catch (e) {
                return ["document", "iframe"].includes(i.destination) ? (console.error(e), function(e, t, r) {
                    throw new Error(e)
                }(e, a, this.address)) : new Response(void 0, {
                    status: 500
                })
            }
        }
        static Ultraviolet = e
    }
    self.UVServiceWorker = i;
    class s {
        constructor(e, t) {
            this.request = e, this.raw = t, this.ultraviolet = e.ultraviolet, this.headers = {};
            for (const e in t.rawHeaders) this.headers[e.toLowerCase()] = t.rawHeaders[e];
            this.status = t.status, this.statusText = t.statusText, this.body = t.body
        }
        get url() {
            return this.request.url
        }
        get base() {
            return this.request.base
        }
        set base(e) {
            this.request.base = e
        }
    }
    class o {
        constructor(e, t, r, i = null) {
            this.ultraviolet = r, this.request = e, this.headers = Object.fromEntries(e.headers.entries()), this.method = e.method, this.address = t.address, this.body = i || null, this.cache = e.cache, this.redirect = e.redirect, this.credentials = "omit", this.mode = "cors" === e.mode ? e.mode : "same-origin", this.blob = !1
        }
        get url() {
            return this.ultraviolet.meta.url
        }
        set url(e) {
            this.ultraviolet.meta.url = e
        }
        get base() {
            return this.ultraviolet.meta.base
        }
        set base(e) {
            this.ultraviolet.meta.base = e
        }
    }
    class n {
        #e;
        #t;
        constructor(e = {}, t = null, r = null) {
            this.#e = !1, this.#t = null, this.data = e, this.target = t, this.that = r
        }
        get intercepted() {
            return this.#e
        }
        get returnValue() {
            return this.#t
        }
        respondWith(e) {
            this.#t = e, this.#e = !0
        }
    }
})();
//# sourceMappingURL=uv.sw.js.map