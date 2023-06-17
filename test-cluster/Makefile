up:
	if ! kind get clusters | grep -q test-cluster; then\
		kind create cluster --name test-cluster --config kind-config.yaml;\
	fi

down:
	if kind get clusters | grep -q test-cluster; then\
		kind delete cluster --name test-cluster;\
	fi