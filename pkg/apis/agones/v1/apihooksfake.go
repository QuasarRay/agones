package v1

import (
	"agones.dev/agones/pkg/apis"
	corev1 "k8s.io/api/core/v1"
	"k8s.io/apimachinery/pkg/util/validation/field"
)

// fakeAPIHooks is a stubabble, fake implementation of APIHooks
// This needs to be private, so it doesn't get picked up by the DeepCopy() generation toolkit.
type fakeAPIHooks struct {
	StubValidateGameServerSpec func(*GameServerSpec, *field.Path) field.ErrorList
	StubValidateScheduling     func(apis.SchedulingStrategy, *field.Path) field.ErrorList
	StubMutateGameServerPod    func(*GameServerSpec, *corev1.Pod) error
	StubSetEviction            func(*Eviction, *corev1.Pod) error
}

var _ APIHooks = fakeAPIHooks{}

// ValidateGameServerSpec is called by GameServer.Validate to allow for product specific validation.
func (f fakeAPIHooks) ValidateGameServerSpec(gss *GameServerSpec, fldPath *field.Path) field.ErrorList {
	if f.StubValidateGameServerSpec != nil {
		return f.StubValidateGameServerSpec(gss, fldPath)
	}
	return nil
}

// ValidateScheduling is called by Fleet and GameServerSet Validate() to allow for product specific validation of scheduling strategy.
func (f fakeAPIHooks) ValidateScheduling(strategy apis.SchedulingStrategy, fldPath *field.Path) field.ErrorList {
	if f.StubValidateScheduling != nil {
		return f.StubValidateScheduling(strategy, fldPath)
	}
	return nil
}

// MutateGameServerPod is called by createGameServerPod to allow for product specific pod mutation.
func (f fakeAPIHooks) MutateGameServerPod(gss *GameServerSpec, pod *corev1.Pod) error {
	if f.StubMutateGameServerPod != nil {
		return f.StubMutateGameServerPod(gss, pod)
	}
	return nil
}

// SetEviction is called by gs.Pod to enforce GameServer.Status.Eviction.
func (f fakeAPIHooks) SetEviction(eviction *Eviction, pod *corev1.Pod) error {
	if f.StubSetEviction != nil {
		return f.StubSetEviction(eviction, pod)
	}
	return nil
}
