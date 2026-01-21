PORTNAME=	woodpecker-plugin-git
DISTVERSION=	2.8.0
CATEGORIES=	devel

MAINTAINER=	ahze@FreeBSD.org
COMMENT=	Git clone plugin for Woodpecker CI
WWW=		https://github.com/woodpecker-ci/plugin-git

LICENSE=	APACHE20
LICENSE_FILE=	${WRKSRC}/LICENSE

RUN_DEPENDS=	git:devel/git

USES=		go:modules

GO_MODULE=	github.com/woodpecker-ci/plugin-git@v0.0.0-20260110112237-7006f95b27dc
GO_TARGET=	.:woodpecker-plugin-git
GO_BUILDFLAGS=	-buildvcs=false -ldflags="-s -w -X main.version=${DISTVERSION}"

PLIST_FILES=	bin/woodpecker-plugin-git

.include <bsd.port.mk>