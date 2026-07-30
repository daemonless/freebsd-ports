PORTNAME=	playwright-core
DISTVERSION=	1.62.1
CATEGORIES=	www

MAINTAINER=	ahze@FreeBSD.org
COMMENT=	Playwright browser automation (FreeBSD native)
WWW=		https://playwright.dev/

LICENSE=	APACHE20
LICENSE_FILE=	${WRKSRC}/LICENSE

RUN_DEPENDS=	chrome:www/chromium \
		ffmpeg:multimedia/ffmpeg

USES=		nodejs:run

MASTER_SITES=	https://registry.npmjs.org/playwright-core/-/
DISTFILES=	playwright-core-${DISTVERSION}.tgz

# npm tarballs extract to package/
WRKSRC=		${WRKDIR}/package

ONLY_FOR_ARCHS=	amd64

NO_BUILD=	yes
SUB_FILES=	playwright
SUB_LIST=	NODE=${LOCALBASE}/bin/node \
		PLATFORM=freebsd-${ARCH}

# 1.60.0 bundled hostPlatform.js + server/registry/index.js into lib/coreBundle.js.
# serverRegistry.js and tools/cli-client/registry.js still exist standalone.
post-patch:
# coreBundle.js carries 3 copies of the "Unsupported platform" throw: two want a
# `return <cachedir>`, one (preceded by `if (!localCacheDir)`) wants an assignment.
# Do the assignment one first (multi-line) so the global throw->return can't clobber it.
	${REINPLACE_CMD} -e '/if (!localCacheDir)/{N;s|throw new Error("Unsupported platform: " + process.platform)|localCacheDir = process.env.XDG_CACHE_HOME \|\| require("path").join(require("os").homedir(), ".cache")|;}' \
		${WRKSRC}/lib/coreBundle.js
	${REINPLACE_CMD} \
		-e 's|function toShortPlatform(hostPlatform2) {|function toShortPlatform(hostPlatform2) { if (hostPlatform2 === "freebsd-${ARCH}") return "freebsd-${ARCH}";|' \
		-e 's|"linux-x64": \["chrome-linux64", "chrome"\],|"linux-x64": ["chrome-linux64", "chrome"], "freebsd-${ARCH}": ["chrome-freebsd${ARCH}", "chrome"],|g' \
		-e 's|"linux-x64": \["chrome-headless-shell-linux64", "chrome-headless-shell"\],|"linux-x64": ["chrome-headless-shell-linux64", "chrome-headless-shell"], "freebsd-${ARCH}": ["chrome-headless-shell-freebsd${ARCH}", "chrome-headless-shell"],|g' \
		-e 's|"linux-x64": \["ffmpeg-linux"\],|"linux-x64": ["ffmpeg-linux"], "freebsd-${ARCH}": ["ffmpeg-freebsd"],|g' \
		-e 's|const suffix = lookAt\[process.platform\]|const _plt = process.platform === "freebsd" ? "linux" : process.platform; const suffix = lookAt[_plt]|g' \
		-e 's|throw new Error("Unsupported platform: " + process.platform)|return process.env.XDG_CACHE_HOME \|\| require("path").join(require("os").homedir(), ".cache")|g' \
		${WRKSRC}/lib/coreBundle.js
	${REINPLACE_CMD} -e \
		's|throw new Error("Unsupported platform: " + process.platform)|return process.env.XDG_CACHE_HOME \|\| require("path").join(require("os").homedir(), ".cache")|' \
		${WRKSRC}/lib/serverRegistry.js
	${REINPLACE_CMD} -e \
		's|throw new Error("Unsupported platform: " + process.platform)|localCacheDir = process.env.XDG_CACHE_HOME \|\| require("path").join(require("os").homedir(), ".cache")|' \
		${WRKSRC}/lib/tools/cli-client/registry.js
	${RM} ${WRKSRC}/lib/coreBundle.js.bak \
		${WRKSRC}/lib/serverRegistry.js.bak \
		${WRKSRC}/lib/tools/cli-client/registry.js.bak

do-install:
	${MKDIR} ${STAGEDIR}${PREFIX}/lib/node_modules/playwright-core
	cd ${WRKSRC} && ${COPYTREE_SHARE} . \
		${STAGEDIR}${PREFIX}/lib/node_modules/playwright-core
	${INSTALL_SCRIPT} ${WRKDIR}/playwright \
		${STAGEDIR}${PREFIX}/bin/playwright

# Browser revisions are read from the bundled browsers.json at install time, so a
# playwright bump needs only DISTVERSION + `make makesum` (no manual CR_REV edit).
# The revisioned symlink paths are appended to the plist here instead of pkg-plist.
post-install:
	@cd ${WRKSRC}; \
	CR_REV=`${AWK} '/"name": "chromium",/{getline;gsub(/[^0-9]/,"");print;exit}' browsers.json`; \
	FFMPEG_REV=`${AWK} '/"name": "ffmpeg",/{getline;gsub(/[^0-9]/,"");print;exit}' browsers.json`; \
	MP=${STAGEDIR}${PREFIX}/libexec/ms-playwright; \
	${MKDIR} $${MP}/chromium-$${CR_REV}/chrome-freebsd${ARCH} \
		$${MP}/chromium_headless_shell-$${CR_REV}/chrome-headless-shell-freebsd${ARCH} \
		$${MP}/ffmpeg-$${FFMPEG_REV}; \
	${LN} -sf ${LOCALBASE}/bin/chrome $${MP}/chromium-$${CR_REV}/chrome-freebsd${ARCH}/chrome; \
	${LN} -sf ${LOCALBASE}/bin/chrome $${MP}/chromium_headless_shell-$${CR_REV}/chrome-headless-shell-freebsd${ARCH}/chrome-headless-shell; \
	${LN} -sf ${LOCALBASE}/bin/ffmpeg $${MP}/ffmpeg-$${FFMPEG_REV}/ffmpeg-freebsd; \
	{ \
		${ECHO_CMD} "libexec/ms-playwright/chromium-$${CR_REV}/chrome-freebsd${ARCH}/chrome"; \
		${ECHO_CMD} "libexec/ms-playwright/chromium_headless_shell-$${CR_REV}/chrome-headless-shell-freebsd${ARCH}/chrome-headless-shell"; \
		${ECHO_CMD} "libexec/ms-playwright/ffmpeg-$${FFMPEG_REV}/ffmpeg-freebsd"; \
		${ECHO_CMD} "@dir libexec/ms-playwright/chromium-$${CR_REV}/chrome-freebsd${ARCH}"; \
		${ECHO_CMD} "@dir libexec/ms-playwright/chromium-$${CR_REV}"; \
		${ECHO_CMD} "@dir libexec/ms-playwright/chromium_headless_shell-$${CR_REV}/chrome-headless-shell-freebsd${ARCH}"; \
		${ECHO_CMD} "@dir libexec/ms-playwright/chromium_headless_shell-$${CR_REV}"; \
		${ECHO_CMD} "@dir libexec/ms-playwright/ffmpeg-$${FFMPEG_REV}"; \
		${ECHO_CMD} "@dir libexec/ms-playwright"; \
	} >> ${TMPPLIST}

.include <bsd.port.mk>
