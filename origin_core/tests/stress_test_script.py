import sys

def main():
    report = []
    
    # Bug 1: Node allows negative threshold which triggers actions on perfect predictions
    try:
        from src.node import Node
        node = Node("n1", expected_traffic=10.0, surprise_threshold=-5.0, surprise_ratio=-0.1)
        node.receive_traffic("src1", 10.0)
        action = node.step()
        if action == "throttle":
            report.append("BUG: Node accepts negative surprise_threshold/ratio and incorrectly throttles on perfect predictions (surprise=0).")
    except Exception as e:
        report.append(f"Node test failed: {e}")

    # Bug 2: LoadGenerator applies multiplier before max(0, traffic), allowing double negatives to generate positive spikes
    try:
        from src.load_generator import LoadGenerator
        gen = LoadGenerator(base_traffic=-10.0, variance=0.0, anomaly_prob=1.0, anomaly_multiplier=-5.0)
        # Without anomaly
        traffic_normal = gen.generate_deterministic(step=1, anomaly_step=2)
        # With anomaly
        traffic_anomaly = gen.generate_deterministic(step=2, anomaly_step=2)
        if traffic_normal == 0.0 and traffic_anomaly == 50.0:
            report.append("BUG: LoadGenerator generates positive anomalies from negative base traffic when anomaly_multiplier is negative.")
    except Exception as e:
        report.append(f"LoadGenerator test failed: {e}")

    # Bug 3: LoadGenerator passes 'variance' parameter as 'sigma' (standard deviation) to random.gauss
    try:
        # Just an observation
        report.append("BUG (Semantic): LoadGenerator passes 'variance' as the second argument to random.gauss(), which expects standard deviation (sigma).")
    except Exception as e:
        pass

    with open("c:\\Users\\ahmad ali\\OneDrive\\Desktop\\Project Origin\\origin_core\\.agents\\teamwork_preview_challenger_m1_gen2_2\\stress_test_report.txt", "w") as f:
        f.write("\n".join(report))

if __name__ == "__main__":
    main()
