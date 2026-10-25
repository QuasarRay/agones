package v1

import "agones.dev/agones/pkg/util/errors"

var gameserverErrors = errors.FromStruct[GameServer](nil)
