import { useState, useEffect, useRef, type ReactNode } from "react";

type Direction = "forward" | "backward" | "commit";

interface StepTransitionProps {
  /** Current step key — changes trigger the transition */
  stepKey: string;
  /** Direction of travel: forward (→), backward (←), or commit (crossfade) */
  direction: Direction;
  children: ReactNode;
}

/**
 * CSS-only step transition wrapper.
 * Stacks the outgoing step (absolute, pointer-events: none) behind the
 * incoming step and applies directional enter/exit animations.
 */
export function StepTransition({ stepKey, direction, children }: StepTransitionProps) {
  const [layers, setLayers] = useState<{
    current: { key: string; content: ReactNode };
    exiting: { key: string; content: ReactNode; dir: Direction } | null;
  }>({
    current: { key: stepKey, content: children },
    exiting: null,
  });

  const prevChildren = useRef(children);
  const prevKey = useRef(stepKey);

  useEffect(() => {
    if (stepKey !== prevKey.current) {
      // Step changed — snapshot the old content as exiting layer
      setLayers({
        current: { key: stepKey, content: children },
        exiting: { key: prevKey.current, content: prevChildren.current, dir: direction },
      });
      prevKey.current = stepKey;
      prevChildren.current = children;
    } else {
      // Same step — just update child content (no animation)
      setLayers((prev) => ({
        ...prev,
        current: { key: stepKey, content: children },
      }));
      prevChildren.current = children;
    }
  }, [stepKey, children, direction]);

  const handleExitEnd = () => {
    setLayers((prev) => ({ ...prev, exiting: null }));
  };

  const dirClass = layers.exiting
    ? `step-transition--${layers.exiting.dir}`
    : "step-transition--forward";

  return (
    <div className={`step-transition ${dirClass}`}>
      {/* Exiting layer — plays exit animation, then removed */}
      {layers.exiting && (
        <div
          className="step-transition__layer step-transition__layer--exiting"
          key={`exit-${layers.exiting.key}`}
          onAnimationEnd={handleExitEnd}
        >
          {layers.exiting.content}
        </div>
      )}

      {/* Entering layer — plays entry animation */}
      <div
        className={`step-transition__layer ${layers.exiting ? "step-transition__layer--entering" : ""}`}
        key={`enter-${layers.current.key}`}
      >
        {layers.current.content}
      </div>
    </div>
  );
}
