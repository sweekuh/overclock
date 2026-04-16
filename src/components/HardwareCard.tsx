interface HardwareCardProps {
  label: string;
  value: string;
  detail?: string;
}

export function HardwareCard({ label, value, detail }: HardwareCardProps) {
  return (
    <div className="hw-card">
      <div className="hw-card__label">{label}</div>
      <div className="hw-card__value">{value}</div>
      {detail && <div className="hw-card__detail">{detail}</div>}
    </div>
  );
}
