"use client";
import { Row } from "@jelper/component";
import $cls from "./index.module.scss";

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <Row align="stretch">
      {/* <Row.Item className={$cls['mod-menu']} fixed width="40px"> 
      </Row.Item> */}
      <Row.Item className={$cls['slide-menu']} fixed width="240px"> 
      </Row.Item>
      <Row.Item>
        {children}
      </Row.Item>
    </Row>
  );
}
