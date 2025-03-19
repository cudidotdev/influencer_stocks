import InfluecerDashboardLayout from "@/components/influencer/dashboard-layout";

export default function Layout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return <InfluecerDashboardLayout>{children}</InfluecerDashboardLayout>;
}
