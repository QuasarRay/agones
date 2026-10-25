package v1

import (
	"agones.dev/agones/pkg/apis"
	corev1 "k8s.io/api/core/v1"
	"k8s.io/apimachinery/pkg/util/validation/field"
)

// APIHooks is a subset of the cloudproduct.CloudProduct interface for cloud product hooks specific to this package.
type APIHooks interface {
	// ValidateGameServerSpec is called by GameServer.Validate to allow for product specific validation.
	ValidateGameServerSpec(*GameServerSpec, *field.Path) field.ErrorList

	// ValidateScheduling is called by Fleet and GameServerSet Validate() to allow for product specific validation of scheduling strategy.
	ValidateScheduling(apis.SchedulingStrategy, *field.Path) field.ErrorList

	// MutateGameServerPod is called by createGameServerPod to allow for product specific pod mutation.
	MutateGameServerPod(*GameServerSpec, *corev1.Pod) error

	// SetEviction is called by gs.Pod to enforce GameServer.Status.Eviction.
	SetEviction(*Eviction, *corev1.Pod) error
}
