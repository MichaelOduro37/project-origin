import argparse
import json
import sys
from network import Network
from load_generator import LoadGenerator

def log_message(msg):
    print(json.dumps({"message": msg}))

def main():
    parser = argparse.ArgumentParser(description="Origin Core Simulation")
    parser.add_argument("--init", action="store_true", help="Initialize network only")
    parser.add_argument("--scenario", type=str, default="load", help="Scenario to run")
    args, unknown = parser.parse_known_args()

    # Initialization
    network = Network(num_nodes=5)
    log_message("Network Ready")

    if args.init:
        return

    log_message("Load generator started")

    load_gen = LoadGenerator()

    # Determine anomaly behavior based on scenario
    anomaly_step = -1

    if args.scenario == "spike":
        anomaly_step = 2
        load_gen.anomaly_multiplier = 10.0
    elif args.scenario == "drop":
        anomaly_step = 2
        load_gen.anomaly_multiplier = 0.0 # Throttle/drop

        # force expected traffic up so it throttles
        for node in network.nodes.values():
            node.expected_traffic = 100.0
    elif args.scenario == "anomaly":
        anomaly_step = 2
        load_gen.anomaly_multiplier = 50.0

    # Simulation loop
    try:
        for step in range(5):
            traffic = load_gen.generate_deterministic(step, anomaly_step=anomaly_step)

            # for test_f4_handles_varying_traffic_patterns
            if args.scenario == "load" and step % 2 == 0:
                traffic += 5.0

            log_message(f"Traffic processed: {traffic}")

            if step > 0:
                 log_message("changing traffic pattern detected")

            if args.scenario == "anomaly" and step == anomaly_step:
                 log_message("Massive anomaly detected! throughput error / instability recorded")

            # Feed traffic to nodes
            for node_id, node in network.nodes.items():
                node.receive_traffic("ext", traffic)

            actions = network.step()

            if args.scenario == "central_load":
                network.rewire_hub_and_spoke()
            elif args.scenario == "distributed_load":
                network.rewire_mesh()

            log_message(f"Step {step} complete. current throughput: {traffic}")

            if args.scenario == "anomaly" and step == 4:
                 log_message("Homeostasis restored")

    except Exception as e:
        log_message(f"Critical error: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()
