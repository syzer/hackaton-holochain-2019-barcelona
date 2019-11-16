/// NB: The try-o-rama config patterns are still not quite stabilized.
/// See the try-o-rama README [https://github.com/holochain/try-o-rama]
/// for a potentially more accurate example

const path = require('path')
const tape = require('tape')

const { Orchestrator, Config, tapeExecutor, singleConductor, combine  } = require('@holochain/try-o-rama')

process.on('unhandledRejection', error => {
  // Will print "unhandledRejection err is not defined"
  console.error('got unhandledRejection:', error);
});

const dnaPath = path.join(__dirname, "../dist/1_hello_holo.dna.json")

const orchestrator = new Orchestrator({
  middleware: combine(
    // squash all instances from all conductors down into a single conductor,
    // for in-memory testing purposes.
    // Remove this middleware for other "real" network types which can actually
    // send messages across conductors
    singleConductor,

    // use the tape harness to run the tests, injects the tape API into each scenario
    // as the second argument
    tapeExecutor(require('tape'))
  ),

  globalConfig: {
    logger: false,
    network: 'memory',
    // OFFICIAL SOLUTION -> but with memory it works as well, so we just leave it
    // network: {
    //   type: 'sim2h',
    //   sim2h_url: 'wss://sim2h.holochain.org:9000',
    // },
  },

  // the following are optional:

  waiter: {
    softTimeout: 5000,
    hardTimeout: 10000,
  },
})

// TODO check if still required
const config = {
  instances: {
    "1_hello_holo": Config.dna(dnaPath, '1_hello_holo'),
  }
}

// test1
orchestrator.registerScenario('Test hello holo TEST1', async (s, t) => {
  const {alice, bob} = await s.players({alice: config, bob: config}, true);
  const result = await alice.call('1_hello_holo', 'hello', 'hello_holo', {"name": "some String"});
  t.ok(result.Ok)
  t.deepEqual(result, { Ok: 'Holo World Tutorial! Hello some String!' })
})

// test2
orchestrator.registerScenario('Test hello holo TEST2', async (s, t) => {
  const {alice, bob} = await s.players({alice: config, bob: config}, true);
  const result = await alice.call('1_hello_holo', 'hello', 'hello_holo', {"name": "John"});
  t.ok(result.Ok)
  t.deepEqual(result, { Ok: 'Holo World Tutorial! Hello John!' })
})

orchestrator.run()
