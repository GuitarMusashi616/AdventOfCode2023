package filter

import "day2/common"

type IPowSet interface {
	Pow(set *common.GameSet) int
}
