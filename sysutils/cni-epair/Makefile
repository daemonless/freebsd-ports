PORTNAME=	cni-epair
DISTVERSIONPREFIX=	v
DISTVERSION=	1.0.0
CATEGORIES=	sysutils

MAINTAINER=	michaeljohnson@ahze.net
COMMENT=	CNI plugin giving containers a LAN address via epair and bridge
WWW=		https://github.com/daemonless/cni-epair

LICENSE=	BSD2CLAUSE
LICENSE_FILE=	${WRKSRC}/LICENSE

RUN_DEPENDS=	${LOCALBASE}/libexec/cni/host-local:net/containernetworking-plugins

USE_GITHUB=	yes
GH_ACCOUNT=	daemonless

# A POSIX shell script: nothing to compile, nothing architecture specific.
NO_ARCH=	yes
NO_BUILD=	yes

PLIST_FILES=	libexec/cni/epair

do-install:
	${MKDIR} ${STAGEDIR}${PREFIX}/libexec/cni
	${INSTALL_SCRIPT} ${WRKSRC}/epair ${STAGEDIR}${PREFIX}/libexec/cni

.include <bsd.port.mk>
