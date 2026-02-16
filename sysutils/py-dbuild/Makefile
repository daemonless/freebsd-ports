PORTNAME=	dbuild
DISTVERSIONPREFIX=	v
DISTVERSION=	2.0.0
CATEGORIES=	sysutils python
PKGNAMEPREFIX=	${PYTHON_PKGNAMEPREFIX}

MAINTAINER=	ahze@FreeBSD.org
COMMENT=	FreeBSD OCI container image build tool
WWW=		https://github.com/daemonless/dbuild

LICENSE=	BSD2CLAUSE
LICENSE_FILE=	${WRKSRC}/LICENSE

BUILD_DEPENDS=	${PYTHON_PKGNAMEPREFIX}setuptools>=63:devel/py-setuptools@${PY_FLAVOR} \
		${PYTHON_PKGNAMEPREFIX}wheel>=0:devel/py-wheel@${PY_FLAVOR}
RUN_DEPENDS=	${PYTHON_PKGNAMEPREFIX}pyyaml>=0:devel/py-pyyaml@${PY_FLAVOR}

USE_GITHUB=	yes
GH_ACCOUNT=	daemonless

USES=		python
USE_PYTHON=	autoplist pep517

NO_ARCH=	yes

OPTIONS_DEFINE=		SCREENSHOT
OPTIONS_DEFAULT=	SCREENSHOT
SCREENSHOT_DESC=	CIT screenshot testing support

SCREENSHOT_RUN_DEPENDS=	${PYTHON_PKGNAMEPREFIX}selenium>=4:www/py-selenium@${PY_FLAVOR} \
			${PYTHON_PKGNAMEPREFIX}scikit-image>=0.23:graphics/py-scikit-image@${PY_FLAVOR} \
			chromedriver:www/chromium

.include <bsd.port.mk>
