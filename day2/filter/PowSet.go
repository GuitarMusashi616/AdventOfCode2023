package filter

import "day2/common"

type PowSet struct {
}

func NewPowSet() *PowSet {
	return &PowSet{}
}

func (p *PowSet) Pow(set *common.GameSet) int {
	res := 1
	for _, count := range set.Cubes {
		res *= count
	}
	return res
}
