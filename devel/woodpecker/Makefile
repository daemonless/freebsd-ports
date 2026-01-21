PORTNAME?=	woodpecker
DISTVERSIONPREFIX=	v
DISTVERSION=	3.13.0
CATEGORIES=	devel

MAINTAINER=	ahze@FreeBSD.org
COMMENT?=	Woodpecker CI
WWW=		https://woodpecker-ci.org/

LICENSE=	APACHE20

.if !defined(PKGNAMESUFFIX)
USES=		metaport
RUN_DEPENDS=	woodpecker-server:devel/woodpecker-server \
		woodpecker-agent:devel/woodpecker-agent \
		woodpecker-cli:devel/woodpecker-cli
.else

USES=		go:1.24+,modules
GO_MODULE=	go.woodpecker-ci.org/woodpecker/v3
GO_PKGNAME=	woodpecker
GO_BUILDFLAGS=	-ldflags="-s -w -X go.woodpecker-ci.org/woodpecker/v3/version.Version=${DISTVERSION}"

WOODPECKER_DB_DIR=	${VARBASE}/db/woodpecker
WOODPECKER_WORK_DIR=	${VARBASE}/db/woodpecker/server
SUB_LIST+=	WOODPECKER_DB_DIR=${WOODPECKER_DB_DIR}
.if ${PKGNAMESUFFIX} == -server
BUILD_DEPENDS=	npm:www/npm
USE_RC_SUBR=	woodpecker-server
SUB_FILES+=	server.conf.sample
GO_TARGET=	./cmd/server:woodpecker-server
USERS=		woodpecker
GROUPS=		woodpecker
ETCDIR=		${PREFIX}/etc/woodpecker
PLIST_FILES=	bin/woodpecker-server \
		"@sample(woodpecker,woodpecker,640) etc/woodpecker/server.conf.sample" \
		"@dir(woodpecker,woodpecker,750) etc/woodpecker" \
		"@dir(woodpecker,woodpecker,750) ${VARBASE}/db/woodpecker" \
		"@dir(woodpecker,woodpecker,750) ${VARBASE}/db/woodpecker/server"

pre-build:
	cd ${WRKSRC}/web && npm install --legacy-peer-deps && npm run build

post-install:
	${MKDIR} -m 750 ${STAGEDIR}${ETCDIR}
	${INSTALL_DATA} ${WRKDIR}/server.conf.sample ${STAGEDIR}${ETCDIR}/
	${MKDIR} -m 750 ${STAGEDIR}${WOODPECKER_DB_DIR}
	${MKDIR} -m 750 ${STAGEDIR}${WOODPECKER_WORK_DIR}
.elif ${PKGNAMESUFFIX} == -agent
WOODPECKER_WORK_DIR=	${VARBASE}/db/woodpecker/agent
SUB_LIST+=	WOODPECKER_WORK_DIR=${WOODPECKER_WORK_DIR}
SUB_FILES+=	agent.conf.sample pkg-message
USE_RC_SUBR=	woodpecker-agent
GO_TARGET=	./cmd/agent:woodpecker-agent
USERS=		woodpecker
GROUPS=		woodpecker
ETCDIR=		${PREFIX}/etc/woodpecker
PLIST_FILES=	bin/woodpecker-agent \
		"@sample(woodpecker,woodpecker,640) etc/woodpecker/agent.conf.sample" \
		"@dir(woodpecker,woodpecker,750) etc/woodpecker" \
		"@dir(woodpecker,woodpecker,750) ${VARBASE}/db/woodpecker" \
		"@dir(woodpecker,woodpecker,750) ${VARBASE}/db/woodpecker/agent"

post-install:
	${MKDIR} ${STAGEDIR}${ETCDIR}
	${INSTALL_DATA} ${WRKDIR}/agent.conf.sample ${STAGEDIR}${ETCDIR}/
	${MKDIR} ${STAGEDIR}${WOODPECKER_WORK_DIR}
.elif ${PKGNAMESUFFIX} == -cli
GO_TARGET=	./cmd/cli:woodpecker-cli
PLIST_FILES=	bin/woodpecker-cli
.endif
.endif

.include <bsd.port.pre.mk>
DIST_SUBDIR=	go/devel_woodpecker/${DISTNAME}
.include <bsd.port.post.mk>
