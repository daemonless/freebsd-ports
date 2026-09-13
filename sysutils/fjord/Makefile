PORTNAME=	fjord
DISTVERSION=	0.2.1
CATEGORIES=	sysutils
MASTER_SITES=	https://github.com/daemonless/${PORTNAME}/releases/download/v${DISTVERSION}/

MAINTAINER=	michaeljohnson@ahze.net
COMMENT=	Web UI for managing container compose stacks
WWW=		https://fjord.daemonless.io

LICENSE=	BSD2CLAUSE
LICENSE_FILE=	${WRKSRC}/LICENSE

USES=		go:modules

GO_TARGET=	./cmd/fjordd:${PREFIX}/sbin/fjordd
GO_LDFLAGS=	-X main.version=${DISTVERSION}

OPTIONS_MULTI=	ENGINE
OPTIONS_MULTI_ENGINE=	APPJAIL PODMAN
OPTIONS_DEFAULT=	APPJAIL PODMAN
ENGINE_DESC=	Container engines (at least one)
APPJAIL_DESC=	AppJail engine (appjail, appjail-director)
PODMAN_DESC=	Podman engine (podman, podman-compose)

APPJAIL_RUN_DEPENDS=	appjail:sysutils/appjail \
		appjail-director:sysutils/py-director
PODMAN_RUN_DEPENDS=	podman:sysutils/podman \
		podman-compose:sysutils/podman-compose \
		catatonit:sysutils/catatonit

PLIST_FILES=	sbin/fjordd \
		etc/rc.d/fjordd

post-patch:
	@${REINPLACE_CMD} 's|/usr/local|${PREFIX}|g' ${WRKSRC}/packaging/fjordd.rc

post-install:
	@${MKDIR} ${STAGEDIR}${PREFIX}/etc/rc.d
	${INSTALL_SCRIPT} ${WRKSRC}/packaging/fjordd.rc \
		${STAGEDIR}${PREFIX}/etc/rc.d/fjordd

.include <bsd.port.mk>
