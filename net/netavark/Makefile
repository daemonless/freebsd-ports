PORTNAME=	netavark
DISTVERSIONPREFIX=	v
DISTVERSION=	2.0.0-164
DISTVERSIONSUFFIX=	-g7e57a492
CATEGORIES=	net

MAINTAINER=	ahze@FreeBSD.org
COMMENT=	Container network stack for podman
WWW=		https://github.com/containers/netavark

LICENSE=	APACHE20
LICENSE_FILE=	${WRKSRC}/LICENSE

BUILD_DEPENDS=	go-md2man:textproc/go-md2man \
		protoc:devel/protobuf
RUN_DEPENDS=	aardvark-dns:dns/aardvark-dns

USES=		cargo
USE_GITHUB=	yes
GH_ACCOUNT=	ahze

.include "${.CURDIR}/Makefile.crates"

CARGO_BUILD_ARGS=	--bin netavark
CARGO_INSTALL=	no

PLIST_FILES=	libexec/podman/netavark \
		sbin/netavark-migrate \
		share/man/man1/netavark.1.gz \
		share/man/man7/netavark-pf.7.gz
PLIST_DIRS=	libexec/netavark

post-build:
.for page in netavark.1 netavark-pf.7
	go-md2man -in ${WRKSRC}/docs/${page}.md -out ${WRKSRC}/docs/${page}
.endfor

do-install:
	${MKDIR} ${STAGEDIR}${PREFIX}/libexec/podman \
		${STAGEDIR}${PREFIX}/libexec/netavark \
		${STAGEDIR}${PREFIX}/sbin
	${INSTALL_PROGRAM} ${CARGO_TARGET_DIR}/${CARGO_BUILD_TARGET}/release/netavark \
		${STAGEDIR}${PREFIX}/libexec/podman
	${SED} -e "s|@PREFIX@|${PREFIX}|g" ${FILESDIR}/netavark-migrate.in \
		> ${WRKDIR}/netavark-migrate
	${INSTALL_SCRIPT} ${WRKDIR}/netavark-migrate \
		${STAGEDIR}${PREFIX}/sbin/netavark-migrate
	${INSTALL_MAN} ${WRKSRC}/docs/netavark.1 ${STAGEDIR}${PREFIX}/share/man/man1
	${INSTALL_MAN} ${WRKSRC}/docs/netavark-pf.7 ${STAGEDIR}${PREFIX}/share/man/man7

.include <bsd.port.mk>
