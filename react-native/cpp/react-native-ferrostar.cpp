// Generated by uniffi-bindgen-react-native
#include "react-native-ferrostar.h"
#include "generated/ferrostar.hpp"

namespace ferrostar {
	using namespace facebook;

	uint8_t installRustCrate(jsi::Runtime &runtime, std::shared_ptr<react::CallInvoker> callInvoker) {
		NativeFerrostar::registerModule(runtime, callInvoker);
		return true;
	}

	uint8_t cleanupRustCrate(jsi::Runtime &runtime) {
		return false;
	}
}