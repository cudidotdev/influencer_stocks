import UserDashboardLayout from "@/components/users/dashboard-layout";

export default function Layout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return <UserDashboardLayout>{children}</UserDashboardLayout>;
}
