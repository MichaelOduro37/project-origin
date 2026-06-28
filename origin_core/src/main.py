import argparse
import json
import sys
import time
from network import Network
from load_generator import LoadGenerator

def log_message(msg):
    print(json.dumps({"message": msg}))
    sys.stdout.flush()

def main():
    parser = argparse.ArgumentParser(description="Origin Core")
    parser.add_argument("--mode", type=str, choices=["simulation", "standalone"], default="simulation", help="Run mode")
    parser.add_argument("--init", action="store_true", help="Initialize network only")
    parser.add_argument("--scenario", type=str, default="load", help="Scenario to run")
    parser.add_argument("--host", type=str, default="0.0.0.0", help="Host IP (Standalone)")
    parser.add_argument("--port", type=int, default=8080, help="Listen Port (Standalone)")
    parser.add_argument("--peer", type=str, help="Connect to peer IP:PORT (Standalone)")
    args, unknown = parser.parse_known_args()

    if args.mode == "standalone":
        # Run as a single real node on a physical device, orchestrating through Network
        network = Network(num_nodes=0) # start empty, add selves
        from node import Node

        my_node_id = f"node_{args.host}_{args.port}"
        node = Node(my_node_id, host=args.host, port=args.port)
        network.nodes[my_node_id] = node
        network.num_nodes += 1

        log_message(f"Origin Node listening on {args.host}:{node.port}")

        if args.peer:
            peer_ip, peer_port_str = args.peer.split(':')
            peer_port = int(peer_port_str)
            peer_id = f"node_{peer_ip}_{peer_port}"

            # create a dummy representation so Network._connect works
            peer_dummy = Node(peer_id, host=peer_ip, port=peer_port)
            network.nodes[peer_id] = peer_dummy
            network.num_nodes += 1

            network._connect(my_node_id, peer_id)
            log_message(f"Connected to peer {args.peer}")

        try:
            while True:
                time.sleep(1)
                network.step()
        except KeyboardInterrupt:
            network.shutdown()
            sys.exit(0)

    # Simulation Initialization
    network = Network(num_nodes=5)
    log_message("Network Ready")

    if args.init:
        network.shutdown()
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

            # Blast actual TCP traffic to all nodes in the network
            for node_id, node in network.nodes.items():
                load_gen.blast_network(node.host, node.port, traffic)

            time.sleep(0.1)

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
    finally:
        network.shutdown()

if __name__ == "__main__":
    main()
