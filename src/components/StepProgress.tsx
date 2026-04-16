interface StepProgressProps {
  current: number;
  total: number;
}

export function StepProgress({ current, total }: StepProgressProps) {
  return (
    <div className="step-progress">
      {Array.from({ length: total }, (_, i) => (
        <div
          key={i}
          className={`step-progress__segment ${
            i < current
              ? "step-progress__segment--complete"
              : i === current
              ? "step-progress__segment--active"
              : ""
          }`}
        />
      ))}
    </div>
  );
}
