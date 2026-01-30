const array = [
  // test comment, should NOT force line break
  foo.a().b(),

  bar.c() // test comment, should force line break
    .d(),

  fizz.e().f()
    .g(), // test comment, should NOT force line break


  buzz.h().i()
    .j() // test comment, should force line break
    .k().l(),

  // test comment, should force line break due to length
	a.b().c().d().e().f().g().h().i().j().k().l().m().n().o().p().q().r().s().t().u().v().w().x().y().z()
]

function test() {
  return (
    foo.bar().fizz() // test comment, should NOT force line break
  )
}

function test2() {
  return (
    foo.bar(). // test comment, should force line break
    fizz().g()
  )
}

// example from issue https://github.com/biomejs/biome/issues/4013
const obj = {
    __init({ commit, state }, { mainToken }) {
        return new Promise((resolve, reject) => {
            utils.tool.http('getMyInfo', {}).then(async (data) => {

                await Promise.all([
                    // NOTICE: Remove this comment and the result will be consistent with prettier
                    utils.tool.http('getGroup').then((groups) => {
                        commit('INIT_GROPUS', groups)
                    }),
                    // NOTICE: Remove this comment and the result will be consistent with prettier
                    utils.tool.http('getChat', {}).then((chats) => {
                        commit('INIT_RECENTCONTACTS', chats)
                    })
                ])

                resolve()
            })
        })
    }
}
