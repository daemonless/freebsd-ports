PORTNAME=	playwright-core
DISTVERSION=	1.59.1
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

# Revisions from browsers.json
CR_REV=		1217
FFMPEG_REV=	1011

PLIST_SUB+=	CR_REV=${CR_REV} \
		FFMPEG_REV=${FFMPEG_REV}

post-patch:
	${REINPLACE_CMD} -e \
		's|function toShortPlatform(hostPlatform2) {|function toShortPlatform(hostPlatform2) { if (hostPlatform2 === "freebsd-${ARCH}") return "freebsd-${ARCH}";|' \
		${WRKSRC}/lib/server/utils/hostPlatform.js
	${REINPLACE_CMD} \
		-e 's|"linux-x64": \["chrome-linux64", "chrome"\],|"linux-x64": ["chrome-linux64", "chrome"], "freebsd-${ARCH}": ["chrome-freebsd${ARCH}", "chrome"],|g' \
		-e 's|"linux-x64": \["chrome-headless-shell-linux64", "chrome-headless-shell"\],|"linux-x64": ["chrome-headless-shell-linux64", "chrome-headless-shell"], "freebsd-${ARCH}": ["chrome-headless-shell-freebsd${ARCH}", "chrome-headless-shell"],|g' \
		-e 's|"linux-x64": \["ffmpeg-linux"\],|"linux-x64": ["ffmpeg-linux"], "freebsd-${ARCH}": ["ffmpeg-freebsd"],|' \
		-e 's|throw new Error("Unsupported platform: " + process.platform)|return process.env.XDG_CACHE_HOME \|\| require("path").join(require("os").homedir(), ".cache")|' \
		-e 's|const suffix = lookAt\[process.platform\]|const _plt = process.platform === "freebsd" ? "linux" : process.platform; const suffix = lookAt[_plt]|' \
		${WRKSRC}/lib/server/registry/index.js
	${REINPLACE_CMD} -e \
		's|throw new Error("Unsupported platform: " + process.platform)|return process.env.XDG_CACHE_HOME \|\| require("path").join(require("os").homedir(), ".cache")|' \
		${WRKSRC}/lib/serverRegistry.js
	${REINPLACE_CMD} -e \
		's|throw new Error("Unsupported platform: " + process.platform)|localCacheDir = process.env.XDG_CACHE_HOME \|\| require("path").join(require("os").homedir(), ".cache")|' \
		${WRKSRC}/lib/tools/cli-client/registry.js

do-install:
	${MKDIR} ${STAGEDIR}${PREFIX}/lib/node_modules/playwright-core
	cd ${WRKSRC} && ${COPYTREE_SHARE} . \
		${STAGEDIR}${PREFIX}/lib/node_modules/playwright-core
	${INSTALL_SCRIPT} ${WRKDIR}/playwright \
		${STAGEDIR}${PREFIX}/bin/playwright
	${MKDIR} ${STAGEDIR}${PREFIX}/libexec/ms-playwright/chromium-${CR_REV}/chrome-freebsd${ARCH}
	${LN} -sf ${LOCALBASE}/bin/chrome ${STAGEDIR}${PREFIX}/libexec/ms-playwright/chromium-${CR_REV}/chrome-freebsd${ARCH}/chrome
	${MKDIR} ${STAGEDIR}${PREFIX}/libexec/ms-playwright/chromium_headless_shell-${CR_REV}/chrome-headless-shell-freebsd${ARCH}
	${LN} -sf ${LOCALBASE}/bin/chrome ${STAGEDIR}${PREFIX}/libexec/ms-playwright/chromium_headless_shell-${CR_REV}/chrome-headless-shell-freebsd${ARCH}/chrome-headless-shell
	${MKDIR} ${STAGEDIR}${PREFIX}/libexec/ms-playwright/ffmpeg-${FFMPEG_REV}
	${LN} -sf ${LOCALBASE}/bin/ffmpeg ${STAGEDIR}${PREFIX}/libexec/ms-playwright/ffmpeg-${FFMPEG_REV}/ffmpeg-freebsd

.include <bsd.port.mk>
